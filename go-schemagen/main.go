//go:build !windows

package main

import (
	"fmt"
	"os"
	"reflect"
	"regexp"
	"strings"
	"unicode"

	"github.com/filecoin-project/go-address"
	"github.com/filecoin-project/go-state-types/abi"
	"github.com/filecoin-project/lotus/api"
	"github.com/filecoin-project/lotus/chain/types"
	"github.com/ipfs/go-cid"
)

type TypeRegistry struct {
	seen        map[reflect.Type]bool
	definitions []string
	nameCount   map[string]int
}

func NewTypeRegistry() *TypeRegistry {
	return &TypeRegistry{
		seen:        make(map[reflect.Type]bool),
		definitions: make([]string, 0),
		nameCount:   make(map[string]int),
	}
}

func toPascalCase(str string) string {
	// Handle empty string
	if str == "" {
		return ""
	}

	// If it already starts with an uppercase letter, return as is
	if len(str) > 0 && unicode.IsUpper(rune(str[0])) {
		return str
	}

	// Otherwise capitalize just the first letter
	return strings.ToUpper(str[:1]) + str[1:]
}

func (tr *TypeRegistry) getUniqueName(baseName string) string {
	if baseName == "" {
		baseName = "EmptyStruct"
	}

	// Convert to PascalCase
	baseName = toPascalCase(baseName)

	tr.nameCount[baseName]++
	if tr.nameCount[baseName] == 1 {
		return baseName
	}
	return fmt.Sprintf("%s%d", baseName, tr.nameCount[baseName]-1)
}

type MethodInfo struct {
	Name           string
	IsSubscription bool
	Params         []ParamInfo
	Returns        []ReturnInfo
}

type ParamInfo struct {
	Name string
	Type string
}

type ReturnInfo struct {
	Type string
}

func debugPrintType(t reflect.Type, depth int) {
	indent := strings.Repeat("  ", depth)
	fmt.Fprintf(os.Stderr, "%sType: %v (Kind: %v)\n", indent, t, t.Kind())

	if t.Kind() == reflect.Struct {
		fmt.Fprintf(os.Stderr, "%sStruct Fields:\n", indent)
		for i := 0; i < t.NumField(); i++ {
			field := t.Field(i)
			fmt.Fprintf(os.Stderr, "%s  Field: %s, Type: %v\n", indent, field.Name, field.Type)
		}
	}

	if t.Kind() == reflect.Interface {
		fmt.Fprintf(os.Stderr, "%sInterface Methods:\n", indent)
		for i := 0; i < t.NumMethod(); i++ {
			method := t.Method(i)
			fmt.Fprintf(os.Stderr, "%s  Method: %s, Type: %v\n", indent, method.Name, method.Type)
		}
	}
}

func getUniqueParamName(baseName string, usedNames map[string]bool) string {
	if baseName == "" {
		baseName = "param"
	}

	finalName := baseName
	counter := 0

	for usedNames[finalName] {
		counter++
		finalName = fmt.Sprintf("%s%d", baseName, counter)
	}

	usedNames[finalName] = true
	return finalName
}

func toSnakeCase(str string) string {
	var matchFirstCap = regexp.MustCompile("(.)([A-Z][a-z]+)")
	var matchAllCap = regexp.MustCompile("([a-z0-9])([A-Z])")

	snake := matchFirstCap.ReplaceAllString(str, "${1}_${2}")
	snake = matchAllCap.ReplaceAllString(snake, "${1}_${2}")
	return strings.ToLower(snake)
}

func getBaseParamName(t reflect.Type) string {
	switch t.Kind() {
	case reflect.Ptr:
		return toSnakeCase(t.Elem().Name())
	case reflect.Slice:
		return toSnakeCase(t.Elem().Name()) + "s"
	case reflect.Map:
		return "map"
	default:
		if t.Name() != "" {
			return toSnakeCase(t.Name())
		}
		return "param"
	}
}

func rustType(t reflect.Type) string {
	// Handle special cases first
	switch t {
	case reflect.TypeOf(cid.Cid{}):
		return "Cid"
	case reflect.TypeOf([]byte{}):
		return "Vec<u8>"
	case reflect.TypeOf(abi.Randomness{}):
		return "String"
	case reflect.TypeOf(types.BigInt{}):
		return "String"
	case reflect.TypeOf(address.Address{}):
		return "String"
	case reflect.TypeOf(types.TipSetKey{}):
		return "Vec<Cid>"
	}

	// Handle channels (for subscriptions)
	if t.Kind() == reflect.Chan {
		if t.ChanDir() == reflect.RecvDir {
			elemType := t.Elem()
			// Handle empty struct case
			if elemType.Kind() == reflect.Struct && elemType.NumField() == 0 {
				return "mpsc::Receiver<()>"
			}
			return fmt.Sprintf("mpsc::Receiver<%s>", rustType(elemType))
		}
		elemType := rustType(t.Elem())
		return fmt.Sprintf("mpsc::Sender<%s>", elemType)
	}

	// Handle empty struct
	if t.Kind() == reflect.Struct && t.NumField() == 0 {
		return "()"
	}

	switch t.Kind() {
	case reflect.Bool:
		return "bool"
	case reflect.Int:
		return "i32"
	case reflect.Int8:
		return "i8"
	case reflect.Int16:
		return "i16"
	case reflect.Int32:
		return "i32"
	case reflect.Int64:
		return "i64"
	case reflect.Uint:
		return "u32"
	case reflect.Uint8:
		return "u8"
	case reflect.Uint16:
		return "u16"
	case reflect.Uint32:
		return "u32"
	case reflect.Uint64:
		return "u64"
	case reflect.Float32:
		return "f32"
	case reflect.Float64:
		return "f64"
	case reflect.String:
		return "String"
	case reflect.Slice:
		if t.Elem().Kind() == reflect.Uint8 {
			return "Vec<u8>"
		}
		return fmt.Sprintf("Vec<%s>", rustType(t.Elem()))
	case reflect.Array:
		return fmt.Sprintf("[%s; %d]", rustType(t.Elem()), t.Len())
	case reflect.Map:
		keyType := rustType(t.Key())
		valueType := rustType(t.Elem())
		return fmt.Sprintf("HashMap<%s, %s>", keyType, valueType)
	case reflect.Ptr:
		return fmt.Sprintf("Option<%s>", rustType(t.Elem()))
	case reflect.Interface:
		if t.NumMethod() == 0 {
			return "Value"
		}
		return "Value"
	case reflect.Struct:
		return toPascalCase(t.Name())
	}

	return "Value"
}

func processMethod(m reflect.Method) MethodInfo {
	method := MethodInfo{
		Name:           m.Name,
		IsSubscription: false,
		Params:         make([]ParamInfo, 0),
		Returns:        make([]ReturnInfo, 0),
	}

	methodType := m.Type
	usedNames := make(map[string]bool)

	// Skip receiver and context parameter
	for j := 2; j < methodType.NumIn(); j++ {
		paramType := methodType.In(j)
		baseName := getBaseParamName(paramType)
		paramName := getUniqueParamName(baseName, usedNames)

		method.Params = append(method.Params, ParamInfo{
			Name: paramName,
			Type: rustType(paramType),
		})
	}

	// Process non-error returns
	for j := 0; j < methodType.NumOut(); j++ {
		returnType := methodType.Out(j)
		if returnType.String() != "error" {
			method.Returns = append(method.Returns, ReturnInfo{
				Type: rustType(returnType),
			})
		}
	}

	// Check if this is a subscription method
	if methodType.NumOut() >= 2 && methodType.Out(0).Kind() == reflect.Chan {
		method.IsSubscription = true
	}

	return method
}

func (tr *TypeRegistry) RegisterType(t reflect.Type) {
	if tr.seen[t] {
		return
	}
	tr.seen[t] = true

	// Register dependent types first
	switch t.Kind() {
	case reflect.Chan:
		// Register the channel's element type
		tr.RegisterType(t.Elem())
	case reflect.Struct:
		for i := 0; i < t.NumField(); i++ {
			tr.RegisterType(t.Field(i).Type)
		}
	case reflect.Slice:
		tr.RegisterType(t.Elem())
	case reflect.Map:
		tr.RegisterType(t.Key())
		tr.RegisterType(t.Elem())
	case reflect.Ptr:
		tr.RegisterType(t.Elem())
	}

	// Generate definition for structs
	if t.Kind() == reflect.Struct {
		def := tr.generateStructDefinition(t)
		if def != "" {
			tr.definitions = append(tr.definitions, def)
		}
	}
}

func (tr *TypeRegistry) generateStructDefinition(t reflect.Type) string {
	// Handle empty struct with no name
	if t.NumField() == 0 && t.Name() == "" {
		return "" // Skip generating empty anonymous structs
	}

	var fields []string
	for i := 0; i < t.NumField(); i++ {
		field := t.Field(i)
		rustFieldName := toRustFieldName(field.Name)
		fieldType := rustType(field.Type)

		jsonTag := field.Tag.Get("json")
		if jsonTag != "" {
			name := strings.Split(jsonTag, ",")[0]
			if name != rustFieldName {
				fields = append(fields, fmt.Sprintf("    #[serde(rename = \"%s\")]\n    pub %s: %s,",
					name,
					rustFieldName,
					fieldType))
			} else {
				fields = append(fields, fmt.Sprintf("    pub %s: %s,",
					rustFieldName,
					fieldType))
			}
		} else {
			if rustFieldName != toSnakeCase(field.Name) {
				fields = append(fields, fmt.Sprintf("    #[serde(rename = \"%s\")]\n    pub %s: %s,",
					toSnakeCase(field.Name),
					rustFieldName,
					fieldType))
			} else {
				fields = append(fields, fmt.Sprintf("    pub %s: %s,",
					rustFieldName,
					fieldType))
			}
		}
	}

	structName := tr.getUniqueName(t.Name())
	return fmt.Sprintf(`#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct %s {
%s
}`, structName, strings.Join(fields, "\n"))
}

var rustKeywords = map[string]bool{
	"as":       true,
	"break":    true,
	"const":    true,
	"continue": true,
	"crate":    true,
	"else":     true,
	"enum":     true,
	"extern":   true,
	"false":    true,
	"fn":       true,
	"for":      true,
	"if":       true,
	"impl":     true,
	"in":       true,
	"let":      true,
	"loop":     true,
	"match":    true,
	"mod":      true,
	"move":     true,
	"mut":      true,
	"pub":      true,
	"ref":      true,
	"return":   true,
	"self":     true,
	"Self":     true,
	"static":   true,
	"struct":   true,
	"super":    true,
	"trait":    true,
	"true":     true,
	"type":     true,
	"unsafe":   true,
	"use":      true,
	"where":    true,
	"while":    true,
	"async":    true,
	"await":    true,
	"dyn":      true,
	"abstract": true,
	"become":   true,
	"box":      true,
	"do":       true,
	"final":    true,
	"macro":    true,
	"override": true,
	"priv":     true,
	"typeof":   true,
	"unsized":  true,
	"virtual":  true,
	"yield":    true,
	"try":      true,
}

func toRustFieldName(name string) string {
	snakeCase := toSnakeCase(name)
	if rustKeywords[snakeCase] {
		return snakeCase + "_"
	}
	return snakeCase
}

func generateStructDefinition(t reflect.Type) string {
	var fields []string
	for i := 0; i < t.NumField(); i++ {
		field := t.Field(i)
		fieldName := field.Name

		// Get rust-safe field name
		rustFieldName := toRustFieldName(fieldName)

		fieldType := rustType(field.Type)
		jsonTag := field.Tag.Get("json")
		if jsonTag != "" {
			name := strings.Split(jsonTag, ",")[0]
			// If the JSON name is different from our rust field name, add rename attribute
			if name != rustFieldName {
				fields = append(fields, fmt.Sprintf("    #[serde(rename = \"%s\")]\n    pub %s: %s,",
					name,
					rustFieldName,
					fieldType))
			} else {
				fields = append(fields, fmt.Sprintf("    pub %s: %s,",
					rustFieldName,
					fieldType))
			}
		} else {
			// If no JSON tag, use the original field name as the rename target
			if rustFieldName != toSnakeCase(fieldName) {
				fields = append(fields, fmt.Sprintf("    #[serde(rename = \"%s\")]\n    pub %s: %s,",
					toSnakeCase(fieldName),
					rustFieldName,
					fieldType))
			} else {
				fields = append(fields, fmt.Sprintf("    pub %s: %s,",
					rustFieldName,
					fieldType))
			}
		}
	}

	return fmt.Sprintf(`#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct %s {
%s
}`, t.Name(), strings.Join(fields, "\n"))
}

func main() {
	var t reflect.Type
	var apiName string

	switch apiArg := os.Args[1]; apiArg {
	case "Common":
		var api struct{ api.Common }
		t = reflect.TypeOf(api)
		apiName = "Common"
	case "FullNode":
		var api struct{ api.FullNode }
		t = reflect.TypeOf(api)
		apiName = "FullNode"
	case "StorageMiner":
		var api struct{ api.StorageMiner }
		t = reflect.TypeOf(api)
		apiName = "StorageMiner"
	case "Gateway":
		var api struct{ api.Gateway }
		t = reflect.TypeOf(api)
		apiName = "Gateway"
	case "Wallet":
		var api struct{ api.Wallet }
		t = reflect.TypeOf(api)
		apiName = "Wallet"
	case "Worker":
		var api struct{ api.Worker }
		t = reflect.TypeOf(api)
		apiName = "Worker"
	default:
		panic("Unknown API")
	}

	fmt.Fprintf(os.Stderr, "=== Analyzing API Type ===\n")
	debugPrintType(t, 0)

	registry := NewTypeRegistry()
	methods := make([]MethodInfo, 0)

	fmt.Fprintf(os.Stderr, "\n=== Analyzing Methods ===\n")
	for i := 0; i < t.NumMethod(); i++ {
		m := t.Method(i)
		fmt.Fprintf(os.Stderr, "Method: %s\n", m.Name)

		method := processMethod(m)
		methods = append(methods, method)

		// Register parameter and return types
		methodType := m.Type
		for j := 1; j < methodType.NumIn(); j++ {
			registry.RegisterType(methodType.In(j))
		}
		for j := 0; j < methodType.NumOut(); j++ {
			registry.RegisterType(methodType.Out(j))
		}
	}

	// Generate Rust code
	fmt.Println("// Code generated by go-schemagen. DO NOT EDIT.")
	fmt.Println()
	fmt.Println("use serde::{Deserialize, Serialize};")
	fmt.Println("use jsonrpc_core::Error;")
	fmt.Println("use serde_json::Value;")
	fmt.Println("use tokio::sync::mpsc;")
	fmt.Println("use std::collections::HashMap;")
	fmt.Println("use uuid::Uuid;")
	fmt.Println("use crate::client::LotusClient;")
	fmt.Println()

	// Output type definitions
	for _, def := range registry.definitions {
		fmt.Println(def)
		fmt.Println()
	}

	// Output API trait
	fmt.Printf("#[async_trait::async_trait]\npub trait %sApi {\n", apiName)
	for _, method := range methods {
		fmt.Printf("    async fn %s(&self", toSnakeCase(method.Name))
		for _, param := range method.Params {
			fmt.Printf(", %s: %s", param.Name, param.Type)
		}
		fmt.Printf(") -> Result<")
		if len(method.Returns) > 0 {
			fmt.Printf("%s", method.Returns[0].Type)
		} else {
			fmt.Printf("()")
		}
		fmt.Printf(", Error>;\n")
	}
	fmt.Println("}")
	fmt.Println()

	// Output Client implementation
	fmt.Printf(`#[derive(Debug, Clone)]
pub struct %sClient {
    client: LotusClient,
}

impl %sClient {
    pub fn new(client: LotusClient) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl %sApi for %sClient {
`, apiName, apiName, apiName, apiName)

	for _, method := range methods {
		fmt.Printf("    async fn %s(&self", toSnakeCase(method.Name))
		for _, param := range method.Params {
			fmt.Printf(", %s: %s", param.Name, param.Type)
		}
		fmt.Printf(") -> Result<")
		if len(method.Returns) > 0 {
			fmt.Printf("%s", method.Returns[0].Type)
		} else {
			fmt.Printf("()")
		}
		fmt.Printf(", Error> {\n")

		if len(method.Params) > 0 {
			fmt.Println("        let params = vec![")
			for _, param := range method.Params {
				fmt.Printf("            serde_json::to_value(&%s).map_err(|e| Error::invalid_params(e.to_string()))?,\n", param.Name)
			}
			fmt.Println("        ];")
		} else {
			fmt.Println("        let params = vec![];")
		}

		if method.IsSubscription {
			fmt.Printf("        self.client.subscribe(\"Filecoin.%s\", params).await", method.Name)
		} else {
			if len(method.Returns) > 0 {
				fmt.Printf("        self.client.request(\"Filecoin.%s\", params).await", method.Name)
			} else {
				fmt.Printf("        self.client.request::<()>(\"Filecoin.%s\", params).await", method.Name)
			}
		}
		fmt.Println()
		fmt.Println("    }")
		fmt.Println()
	}

	fmt.Println("}")
}

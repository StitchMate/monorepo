package main

import (
	"encoding/xml"
	"errors"
	"fmt"
	"net/http"

	"github.com/globusdigital/soap"
	"github.com/go-playground/validator/v10"
)

// FooRequest a simple request
type FooRequest struct {
	XMLName xml.Name `xml:"fooRequest"`
	Foo     string   `validate:"required"`
}

// FooResponse a simple response
type FooResponse struct {
	Bar string
}

var validate *validator.Validate

func init() {
	validate = validator.New()
}

// RunServer run a little demo server
func RunServer() {
	soapServer := soap.NewServer()
	soapServer.RegisterHandler(
		"/",
		// SOAPAction
		"operationFoo",
		// tagname of soap body content
		"fooRequest",
		// RequestFactoryFunc - give the server sth. to unmarshal the request into
		func() interface{} {
			return &FooRequest{}
		},
		// OperationHandlerFunc - do something
		func(request interface{}, w http.ResponseWriter, httpRequest *http.Request) (response interface{}, err error) {
			fooRequest := request.(*FooRequest)
			if err := validate.Struct(fooRequest); err != nil {
				// Convert validation errors to error message
				if validationErrors, ok := err.(validator.ValidationErrors); ok {
					var errorMsg string
					for _, e := range validationErrors {
						errorMsg += fmt.Sprintf("Field: %s, Error: %s; ", e.Field(), e.Tag())
					}
					return nil, errors.New(errorMsg)
				}
				return nil, err
			}

			fooResponse := &FooResponse{
				Bar: "Hello " + fooRequest.Foo,
			}
			return fooResponse, nil
		},
	)
	err := http.ListenAndServe(":8080", soapServer)
	fmt.Println("exiting with error", err)
}

func main() {
	RunServer()
}

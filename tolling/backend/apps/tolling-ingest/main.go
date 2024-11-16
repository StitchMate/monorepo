package main

import (
	"encoding/xml"
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

type SOAPFault struct {
	XMLName     xml.Name     `xml:"http://schemas.xmlsoap.org/soap/envelope/ Fault"`
	FaultCode   string       `xml:"faultcode"`
	FaultString string       `xml:"faultstring"`
	Detail      *FaultDetail `xml:"detail,omitempty"`
}

type FaultDetail struct {
	ValidationErrors []ValidationError `xml:"ValidationError"`
}

type ValidationError struct {
	Field    string `xml:"field"`
	Reason   string `xml:"reason"`
	Location string `xml:"location"`
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
					fault := &SOAPFault{
						FaultCode:   "soap:Client",
						FaultString: "Multiple validation errors occurred",
						Detail: &FaultDetail{
							ValidationErrors: make([]ValidationError, len(validationErrors)),
						},
					}
					for i, verr := range validationErrors {
						fault.Detail.ValidationErrors[i] = ValidationError{
							Field:    verr.Field(),
							Reason:   verr.Tag(),
							Location: fmt.Sprintf("/tollRequest/%s", verr.Field()),
						}
					}
					return fault, nil
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

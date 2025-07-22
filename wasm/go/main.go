package main

import (
	"encoding/json"
	"time"

	"github.com/clarkmcc/cron"
	"github.com/extism/go-pdk"
)

// Request structure for parsing a cron expression
type ParseRequest struct {
	Expression string `json:"expression"`
	Options    int    `json:"options,omitempty"`
}

// Request structure for getting next/prev times
type TimeRequest struct {
	Expression string `json:"expression"`
	Timestamp  int64  `json:"timestamp"`
	Options    int    `json:"options,omitempty"`
}

// Response structure for success/error
type Response struct {
	Success   bool   `json:"success,omitempty"`
	Error     string `json:"error,omitempty"`
	Timestamp int64  `json:"timestamp,omitempty"`
}

//export parse_schedule
func parse_schedule() int32 {
	// Get input from the host
	input := pdk.Input()

	// Parse the request
	var req ParseRequest
	if err := json.Unmarshal(input, &req); err != nil {
		return writeError("Invalid request format: " + err.Error())
	}

	if req.Expression == "" {
		return writeError("Missing cron expression")
	}

	p := cron.NewParser(cron.ParseOption(req.Options))

	// Parse the cron expression
	_, err := p.Parse(req.Expression)
	if err != nil {
		return writeError("Failed to parse cron expression: " + err.Error())
	}

	// Return success
	resp := Response{Success: true}
	return writeResponse(resp)
}

//export get_next_time
func get_next_time() int32 {
	// Get input from the host
	input := pdk.Input()

	// Parse the request
	var req TimeRequest
	if err := json.Unmarshal(input, &req); err != nil {
		return writeError("Invalid request format: " + err.Error())
	}

	if req.Expression == "" {
		return writeError("Missing cron expression")
	}

	// Create a parser with the provided options or use the default parser
	p := cron.NewParser(cron.ParseOption(req.Options))

	// Parse the cron expression
	schedule, err := p.Parse(req.Expression)
	if err != nil {
		return writeError("Failed to parse cron expression: " + err.Error())
	}

	// Calculate the next time
	t := time.Unix(req.Timestamp, 0)
	next := schedule.Next(t)

	// Return the result
	resp := Response{
		Success:   true,
		Timestamp: next.Unix(),
	}
	return writeResponse(resp)
}

//export get_prev_time
func get_prev_time() int32 {
	// Get input from the host
	input := pdk.Input()

	// Parse the request
	var req TimeRequest
	if err := json.Unmarshal(input, &req); err != nil {
		return writeError("Invalid request format: " + err.Error())
	}

	if req.Expression == "" {
		return writeError("Missing cron expression")
	}

	// Create a parser with the provided options or use the default parser
	p := cron.NewParser(cron.ParseOption(req.Options))

	// Parse the cron expression
	schedule, err := p.Parse(req.Expression)
	if err != nil {
		return writeError("Failed to parse cron expression: " + err.Error())
	}

	// Calculate the previous time
	t := time.Unix(req.Timestamp, 0)
	prev := schedule.Prev(t)

	// Return the result
	resp := Response{
		Success:   true,
		Timestamp: prev.Unix(),
	}
	return writeResponse(resp)
}

// Helper function to write an error response
func writeError(message string) int32 {
	resp := Response{Error: message}
	return writeResponse(resp)
}

// Helper function to write a response
func writeResponse(resp Response) int32 {
	data, err := json.Marshal(resp)
	if err != nil {
		// If we can't marshal the response, create a simple error message
		errorData := []byte(`{"error":"Failed to marshal response"}`)
		mem := pdk.AllocateBytes(errorData)
		pdk.OutputMemory(mem)
		return 0
	}

	// Allocate memory for the response and set it as the output
	mem := pdk.AllocateBytes(data)
	pdk.OutputMemory(mem)
	return 0
}

func main() {
	// This function is required but not used with Extism
}

package main

type QuoteReport struct {
	ID                    string `json:"id"`
	Timestamp             string `json:"timestamp"`
	Version               int    `json:"version"`
	IsvEnclaveQuoteStatus string `json:"isvEnclaveQuoteStatus"`
	PlatformInfoBlob      string `json:"platformInfoBlob"`
	IsvEnclaveQuoteBody   string `json:"isvEnclaveQuoteBody"`
}

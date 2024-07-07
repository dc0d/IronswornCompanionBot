package oracles

type Oracle struct {
	Title   string        `json:"Title"`
	Oracles []OracleTable `json:"Oracles"`
}

type OracleTable struct {
	Name        string              `json:"Name"`
	OracleTable []ChanceDescription `json:"Oracle Table"`

	Oracles     []OracleTable `json:"Oracles,omitempty"`
	D           string        `json:"d,omitempty"`
	Chance      int           `json:"Chance,omitempty"`
	Description string        `json:"Description,omitempty"`
	Prompt      string        `json:"Prompt,omitempty"`
}

type ChanceDescription struct {
	Chance      int    `json:"Chance"`
	Description string `json:"Description"`
}

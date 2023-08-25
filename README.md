This is the main server for marseadata build using Rust for reliability, speed and size.

## API document

To reach the API, we use the url:
`[address]/api/search`

The formate of the API is as follow:

```json
{
  "ItemOne": {
    "column": "name-of-column",
    "operator": "regexp",
    "pattern": "regexp-pattern"
  },
  "ItemTwo": {
    "column": "name-of-column-2",
    "operator": "regexp",
    "pattern": "regexp-pattern"
  }
}
```

For example:

```json
{
  "ID": {
    "column": "ID",
    "operator": "regexp",
    "pattern": "."
  }
}
```
This will return all entry.

```json
{
  "ID": {
    "column": "Year Published",
    "operator": "regexp",
    "pattern": "2018"
  }
}
```
This will return all paper that is published in 2018.

Caution, please refer to the schema below, to correctly use the right column name, as a wrong a query could crash the server. This is due to the temporary structure of the server API. Until we have a set of standard for more specific API later on, the server is a bit unstable to let room for fast modification and avoid premature optimization.

```typescript
const schemaDB: PropertiesSchema = {
  ID: "string" || null,
  "Sci/Humanities": "string" || null,
  "Year Published": "string" || null,
  "Geographical Scale": "string" || null,
  "Location/Territory studied": "string" || null,
  Type: "string" || null,
  "Link to source": "string" || null,
  Language: "string" || null,
  Citation: "string" || null,
  Title: "string" || null,
  "Translated Title": "string" || null,
  "Author(s)": "string" || null,
  "First Author": "string" || null,
  "Corresponding Author": "string" || null,
  Journal: "string" || null,
  "Editor(s)": "string" || null,
  "Book Title": "string" || null,
  "Research Group(s)": "string" || null,
  "Country/territory of Research Institution": "string" || null,
  "Funding Information": "string" || null,
  "Research Scope Aim of Research": "string" || null,
  "Period of Study": "string" || null,
  "Period of Study_Year": "string" || null,
  "Location of Work": "string" || null,
  "Relevant Water Body_Detailed": "string" || null,
  "Relevant Water Body_General": "string" || null,
  "Coastal or Offshore": "string" || null,
  "Plastic Sizes Examined": "string" || null,
  "Adopted GESAMP Size": "string" || null,
  "Microplastic Sizes": "string" || null,
  "Contaminants Examined": "string" || null,
  "Fishing Gear Examined": "string" || null,
  "Legal/Regulatory Study": "string" || null,
  "Social/Cultural Study": "string" || null,
  "Economic/Management Study": "string" || null,
  "Policy Study": "string" || null,
  "Research Methodology Methodologies Used": "string" || null,
  "Field Sampling_Conducted": "string" || null,
  "Field Sampling_Compartment": "string" || null,
  "Field Sampling_Frequency": "string" || null,
  "Survey/Interview_Conducted": "string" || null,
  "Other Sampling_Type": "string" || null,
  Biota_Species: "string" || null,
  "Biota (Phyllum)": "string" || null,
  Biota_Applied: "string" || null,
  "Common names": "string" || null,
  "Literature Review_Conducted": "string" || null,
  "Literature Review_Volume": "string" || null,
  "Desktop / Deductive analysis": "string" || null,
  Modelling_Conducted: "string" || null,
  Modelling_Type: "string" || null,
  "Plankton Net_Mesh Size": "string" || null,
  "Water Sampling_Depth": "string" || null,
  "Shoreline Sediment Sampling_ Depth": "string" || null,
  "Seabed Sediment Sampling_Depth": "string" || null,
  "Mangrove/Mudflat Sediment Sampling_Depth": "string" || null,
  Controls_Blanks: "string" || null,
  "Research Findings Key Findings": "string" || null,
  "Source of Plastics": "string" || null,
  "Source of Plastics_General": "string" || null,
  "Research Topics": "string" || null,
  "Plastic Characterisation_Conducted": "string" || null,
  "Plastic Characterisation_Colour": "string" || null,
  "Plastic Characterisation_Colours Found": "string" || null,
  "Plastic Characterisation_Shape": "string" || null,
  "Plastic Characterisation_Shapes Found": "string" || null,
  "Plastic Characterisation_Polymer": "string" || null,
  "Plastic Characterisation_Polymers Found": "string" || null,
  Macro_Uses: "string" || null,
  "Macro_Mean Abundance_Count": "string" || null,
  "Macro_Mean Abundance_Weight": "string" || null,
  "Water_Mean Abundance_Count": "string" || null,
  "Water_Mean Abundance_Weight": "string" || null,
  "Shoreline Sediment_Mean Abundance_Count": "string" || null,
  "Shoreline Sediment_Mean Abundance_Weight": "string" || null,
  "Seabed Sediment_Mean Abundance_Count": "string" || null,
  "Seabed Sediment_Mean Abundance_Weight": "string" || null,
  "Mangrove_Mean Abundance_Count": "string" || null,
  "Mangrove_Mean Abundance_Weight": "string" || null,
  "Biota_Mean Abundance_Count": "string" || null,
  "Biota_Mean Abundance_Weight": "string" || null,
  "Degradation Indicated": "string" || null,
};
```

## Docker setup:

- build command 
  ```shell
  docker build -t huy2840/neo-server .
  ```
- run command
  ```shell
  docker run -it --rm -d -p 8000:8000 huy2840/neo-server
  ```
- attach to docker for inspection 
  ```shell
  docker container exec -it container-name /bin/bash
  ```

Replace the image name above to your own image name.

## cron.sh file:

```bash
#!/bin/bash

docker stop $(docker ps -a -q --filter ancestor=huy2840/neo-server --format="{{.ID}}")
docker run -it --rm -d -p 8000:8000 huy2840/neo-server

echo "Restart server at:" >> cron-log.txt
TZ='Asia/Singapore' date >> cron-log.txt
```

if you intend to build and push your own docker container server, remember to change the `ancestor=huy2840/neo-server` to your own container-image-name.

## cron task schedule:
```bash
5 1 * * sun /root/cron.sh
```
Which reset the server at 1:05 every Sunday.


# Insurer Price Transparency

The file `machine_readable_links` provides links to known MRFs for insurer price transparency. It is a flat file
with the following schema:

| Name | Description | Type | Definition | Required |Example Value |
| ----- | ---- | ---- | ---------- | -------- | -------- |
| **reporting_entity_name_legal** | Entity Name (legal) | String | The legal name of the entity publishing the machine-readable file, i.e. the insurer's legal name. | No | |
| **reporting_entity_name_common** | Entity Name (common) | String | The common name of the entity publishing the machine-readable file, i.e. the insurer's common name. | Yes | United Healthcare |
| **reporting_entity_type** | Entity Type | Enum("insurer", "other") | The type of entity that is publishing the machine-readable file. | No | insurer |
| **machine_readable_url** | Machine Readable URL | String | A (purported) url for the machine readable file resource. | Yes | transparency-in-coverage.uhc.com |
| **machine_readable_url_status** | Machine Readable URL Status | Enum("up", "down", "corrupt") | A status code for the purported url for the machine readable file resource. | Yes | down |
| **machine_readable_page** | Consumer Page URL | String | URL for an official consumer facing page containing a link to the MRF. | No | |
| **supplemental_url** | Supplemental URL | String | A url for any supplemental information pertaining to the particular transparency MRF in question. | No | https://www.uhc.com/content/dam/uhcdotcom/en/HealthReform/PDF/Provisions/reform-external-transparancy-FAQs.pdf |
| **file_name** | File Name | String | Default name of file served | No | |
| **file_format** | File Format | Enum("csv", "json", "xml", "other") | Format of the file. | Yes | other |
| **meets_standard** | Meets Standard | Enum("true", "false") | Whether the MRF name and format meet the required standard. | No | |
| **standard_issue** | Standard Issue | String | If standard is not met, a description of the discrepancy. | No | |
| **state_or_region** | State or Region | Enum(ABBREV) | State or region in which legal reporting entity is incorporated. | No | |
| **last_updated_date** | Last Updated Date | String | The date in which **this** record (not the MRF) was last updated. Date must be in an ISO 8601 format (i.e. YYYY-MM-DD). | Yes | 2022-04-14 |

**Note:**

The state_or_region entry, if entered, must be one of the following ABBREV standardized abbreviation strings:

| State or Region | ABBREV |
| ----------        | --------- |
| Alabama |AL |
|Alaska | AK |
|American Samoa | AS |
|Arizona | AZ |
|Arkansas | AR |
|California | CA |
|Colorado | CO |
|Connecticut |CT |
|Delaware | DE
|District of Columbia | DC|
|Florida | FL|
|Georgia | GA|
| Guam | GU |
|Hawaii | HI|
|Idaho | ID|
|Illinois | IL|
|Indiana | IN|
|Iowa | IA|
|Kansas | KS|
|Kentucky | KY|
|Louisiana | LA|
|Maine | ME|
|Maryland | MD|
|Massachusetts | MA|
|Michigan | MI|
|Minnesota | MN|
|Mississippi | MS|
|Missouri | MO|
|Montana | MT|
|Nebraska | NE|
|Nevada| NV|
|New Hampshire | NH|
|New Jersey | NJ|
|New Mexico | NM|
|New York | NY|
|North Carolina | NC|
|North Dakota | ND|
| Northern Mariana Islands | MP |
|Ohio | OH|
|Oklahoma | OK|
|Oregon | OR|
|Pennsylvania | PA|
| Puerto Rico | PR |
|Rhode Island | RI|
|South Carolina | SC|
|South Dakota | SD|
|Tennessee | TN|
|Texas | TX|
|Utah | UT|
|Vermont | VT|
| Virgin Islands | VI |
|Virginia | VA|
|Washington | WA|
| West Virginia | WV |
|Wisconsin | WI|
|Wyoming | WY|
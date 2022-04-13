# Insurer Price Transparency

The file `machine_readable_links` provides links to known MRFs for hospital price transparency. It is a flat file
with the following schema:

| Name | Description | Type | Definition | Required |
| ----- | ---- | ---- | ---------- | -------- |
| **reporting_entity_name_legal** | Entity Name (legal) | String | The legal name of the entity publishing the machine-readable file, i.e. the hospital's legal name. | Yes |
| **reporting_entity_name_common** | Entity Name (common) | String | The common name of the entity publishing the machine-readable file, i.e. the hospital's common name. | No |
| **reporting_entity_type** | Entity Type | String | The type of entity that is publishing the machine-readable file. | No |
| **machine_readable_url** | Machine Readable URL | String | A (purported) url for the machine readable file resource. | Yes |
| **machine_readable_url_status** | Machine Readable URL Status | Enum("up", "down", "corrupt") | A status code for the purported url for the machine readable file resource. | Yes |
| **machine_readable_page** | Consumer Page URL | String | URL for an official consumer facing page containing a link to the MRF | No |
| **supplemental_url** | Supplemental URL | String | A url for any supplemental information pertaining to the particular transparency MRF in question. | No |
| **file_name** | File Name | String | Default name of file served | No |
| **file_format** | File Format | Enum("csv", "json", "xml") | Format of the file. | Yes |
| **meets_standard** | Meets Standard | Enum("true", "false") | Whether the MRF name and format meet the required standard. | No |
| **standard_issue** | Standard Issue | String | If standard is not met, a description of the discrepancy. | No |
| **state_or_region** | State or Region | String | State or region in which legal reporting entity is incorporated. | No |
| **last_updated_date** | Last Updated Date | String | The date in which **this** record (not the MRF) was last updated. Date must be in an ISO 8601 format (i.e. YYYY-MM-DD) | Yes |
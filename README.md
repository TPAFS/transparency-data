<!-- [![Persius Transparency Data](assets/logo512.png)](#) -->
<!-- <img src="assets/logo512.png" width="200" height="200" /> -->

[![Persius Transparency Data Outro Banner](assets/banner.webp?raw=true)](#)

# Transparency Data

This is a data repository for transparency related data in healthcare, produced and maintained by Persius.

The goal of this resource is to provide easily accessible data related to the existence, practices
and costs of hospitals and health insurance issuers in the United States. Our hope is that with enough data, U.S. consumers,
advocacy groups, and other third parties will be empowered to end unjust practices in healthcare, drive costs down,
and work towards equitable, accessible government-managed care for all. At the least, we hope that enough easily accessible data will ensure the worst abusers of our systems are recognized and held responsible by the people.

## Contents
<!-- TODO: Figure out a method of creating TOC that actually works for github markdown rendering. -->

- [Price Transparency](#price-transparency)
  - [Hospitals](#hospitals)
    - [Machine Readable Files](#machine-readable-files)
    - [Report Violations](#report-violations)
  - [Insurance Issuers](#insurance-issuers)
    - [Machine Readable Files](#machine-readable-files-1)
    - [Report Violations](#report-violations-1)
- [Practices Transparency](#practices-transparency)
  - [Hospitals](#hospitals-1)
  - [Insurance Issuers](#insurance-issuers-1)
- [Existence Transparency](#existence-transparency)
  - [Hospitals](#hospitals-2)
  - [Insurance Issuers](#insurance-issuers-2)
- [Contributing](#contributing)
- [Disclaimer](#disclaimer)
- [Citation](#citation)
- [Contact](#contact)

## Price Transparency

Under the Affordable Care Act (ACA), the Centers for Medicare and Medicaid Services (CMS) and the Department of Health and Human Services (HHS) have produced and implemented rules and regulations aimed at providing transparent access to pricing data across U.S. healthcare. There are many exciting developments in the scope and implications of these rules that are coming to fruition right now.

This repository is meant to serve as a supplement to the raw data that the transparency rules require organizations to serve to the public, and the consumer facing websites and resources provided by CMS and HHS. In particular, there are certain problems with the current implementations of these rules that leave gaps and pose barriers to effective use of the transparency data by the general public, and we hope to fill those gaps and lower those barriers.

### Hospitals

The rules implemented by CMS and HHS pertaining to hospitals require hospitals to provide public facing pricing data for certain standard charges and services. In short, each hospital must serve a file online, accessible by the public, that details a menu (with prices) for some of the services they provide. They also must honor (to an extent :/) the prices listed in that file. The file is referred to as a Machine Readable File (MRF). Price transparency of hospital standard charges rules are defined in CMS rule CMS-1717-F2 (originally published 11/15/2019). The final rules went into effect on 1/1/2021.

You can read about these Hospital Price Transparency rules in the following locations:

- [Original CMS Press Release](https://www.cms.gov/newsroom/fact-sheets/cy-2020-hospital-outpatient-prospective-payment-system-opps-policy-changes-hospital-price)

- [CMS Fact Sheet](https://www.hhs.gov/sites/default/files/cms-1717-f2.pdf)

- [Federal Register Description (1)](https://www.federalregister.gov/documents/2019/11/27/2019-24931/medicare-and-medicaid-programs-cy-2020-hospital-outpatient-pps-policy-changes-and-payment-rates-and#p-1010)

- [Federal Register Description (2)](https://www.govinfo.gov/content/pkg/FR-2019-11-27/pdf/2019-24931.pdf)

CMS also maintains a [consumer facing webpage](https://www.cms.gov/hospital-price-transparency) about the rules.

#### Machine Readable Files

To get utility from the machine readable files provided by hospitals, one has to know they exist, and where to find them. If one knows in advance that they only care about one particular hospital, it is relatively straightforward to do some searching to discover the location of the relevant file (assuming the hospital is in compliance). However, this sort of use case is, in our opinion, one where the transparency file has relatively limited utility. Knowing prices in advance is useful for many reasons, but one of the most promising reasons is because it allows consumers to _compare_ prices across providers, and proactively choose their provider based on prices (i.e. to "shop around").

This latter sort of use case is less straightforward given the current rule implementation, because (as far as we know) there is no centralized, government-run website that maintains a list of all hospitals' transparency urls (nor have we seen indication that anyone is planning to produce such a list in the immediate future). This means that to "shop around", a consumer has to compile a list of hospitals they are considering, perform a search to find the MRF for each one, and then open each of those files and cross reference between them.

Having the urls for all such files in one location would be invaluable to consumers, and third parties building tools to support consumers, since it would allow people to easily download and compare many files, and aggregate the pricing findings in one location, all from one source (or easily write software to do this!). There are plenty of companies doing this aggregation themselves, keeping the aggregate lists proprietary, and selling the aggregated info that allows consumers to "shop around" back to consumers and ealthcare professionals. While it is great these resources exist, we believe they do not do enough. The fact that these companies are not open sourcing the list of MRF urls they collect goes against the entire intention of the rules, which are meant to provide clear and easy access _directly to consumers_, for free. Existing resources contribute more costs to our systems by adding more stockholders looking for profits from the very data that was meant to allow consumers to spend less, and make our systems more broadly accessible and efficient. Since we as consumers pay taxes to fund all of the work CMS and HHS do, it is reasonable to consider the MRFs and other transparency data that providers are being required to submit as belonging to _us_, the people, and we should work to ensure it exists in it's full glory outside proprietary silos that drive all of our costs up.

We will try to maintain a centralized source of truth for the urls of publicly accessible MRFs for hospitals in the U.S. This data can be viewed in [`price_transparency/hospitals/machine_readable_links.csv`](./price_transparency/hospitals/machine_readable_links.csv), and you can read about the schema of that file in [`price_transparency/hospitals/README.md`](./price_transparency/hospitals/README.md).

Here is an example of the data in this file:

| reporting_entity_name_legal| reporting_entity_name_common | reporting_entity_type | machine_readable_url | machine_readable_url_status | machine_readable_page | supplemental_url | file_name | file_format | meets_standard | standard_issue | state_or_region | last_updated_date |
| ---| --- | --- | ---- | ---- | --- |---- | --- | --- | --- | --- | --- | --- |
| | WellSpan York Hospital | hospital | https://www.wellspan.org/media/2659837/231352222_york-hospital_standardcharges.json | up | https://www.wellspan.org/patients-visitors/patient-guide/billing-insurance/cost-of-care/ | | 231352222_york-hospital_standardcharges.json | json| | | PA | 2022-05-09 |
 | |  Wills Eye Hospital |hospital | https://www.willseye.org/wp-content/uploads/2021/01/Wills-Eye-Hospital-standardcharges.csv | up | https://www.willseye.org/patients-visitors/current-standard-charges/| | Wills-Eye-Hospital-standardcharges.csv| csv | | |PA | 2022-05-09|

**Note:** While there is no centralized, government-run site that includes a list of all MRF urls, there is documentation provided by CMS about what form the file names of these MRFs should take, which might prove useful. Namely, in [this CMS guide](https://www.cms.gov/files/document/steps-machine-readable-file.pdf)
they describe that file name should follow the template:
`<ein>_<hospital-name>_standardcharges.[json|xml|csv]`. If we had a full list of all hospital names and EINs in the U.S., and we could be sure each of them was following this rule strictly, this would be a way for us to obtain all of the MRF file names very easily. If we additionally knew how those files were being served in some consistent manner relative to a list of known hospital owned domains, we could automate the work we seek to accomplish in this subsection. This is an example of how lack of _existence transparency_ data can limit the utility of other data. See Existence Transparency for more details below.

You can read details and structure of the required pricing files via CMS documentation.

#### Report Violations

The final rules for hospital price transparency are currently in effect! Since the rules are new and enforcement has so far been limited, it is unclear how many hospitals are in strict compliance. If you observe hospitals who are not adhering to the requirements
of the final rule, you should [contact CMS directly](https://www.cms.gov/hospital-price-transparency/contact-us) to report the violation.

### Insurance Issuers
Just as there are rules implemented by CMS and HHS requiring pricing transparency from hospitals, there are also rules requiring pricing transparency from health insurers. These rules require group health plans and health insurance issuers to disclose certain pricing and cost-sharing information to the public. In
particular, this includes providing cost-sharing information for certain covered services for particular providers, which combined with hospital pricing transparency can be invaluable to consumers trying to understand their out of pocket costs for particular services, at particular facilities, with particular insurance. Insurers must provide this cost information via MRFs, Price transparency rules for health insurers are defined in HHS rule 85 FR 72158  (originally published 11/27/2019). The final rules effectively start on 7/1/2022.

You can read about the transparency rules for insurers in the following locations:

- [Federal Register Description (1)](https://www.federalregister.gov/documents/2019/11/27/2019-25011/transparency-in-coverage)
- [Federal Register Description (2)](https://www.govinfo.gov/app/details/FR-2020-11-12/2020-24591)
- [CMS Consumer Webpage](https://www.cms.gov/healthplan-price-transparency)
- [CMS MRF technical specification](https://github.com/CMSgov/price-transparency-guide)

#### Machine Readable Files

Our goal for insurance pricing transparency is to maintain a centralized source of truth for the urls of publicly accessible MRFs for insurers in the U.S. This data can be viewed in [`price_transparency/insurers/machine_readable_links.csv`](./price_transparency/insurers/machine_readable_links.csv), and you can read about the schema of that file in [`price_transparency/insurers/README.md`](./price_transparency/insurers/README.md).

Here is an example of the data in this file:

| reporting_entity_name_legal| reporting_entity_name_common | reporting_entity_type | machine_readable_url | machine_readable_url_status | machine_readable_page | supplemental_url | file_name | file_format | meets_standard | standard_issue | state_or_region | last_updated_date |
| ---| --- | --- | ---- | ---- | --- |---- | --- | --- | --- | --- | --- | --- |
| United Healthcare | United Healthcare| insurer |transparency-in-coverage.uhc.com | down |transparency-in-coverage.uhc.com |https://www.uhc.com/content/dam/uhcdotcom/en/HealthReform/PDF/Provisions/reform-external-transparancy-FAQs.pdf||||||2022-04-13

**Note:** As for hospitals, CMS outlines standard format for the file names for various insurer price transparency files that are required. Check out the CMS technical specification repo [here](https://github.com/CMSgov/price-transparency-guide) for more details.

#### Report Violations

**Coming soon.**

## Practices Transparency

**Coming soon.**
<!-- TODO: Describe the broad goal of practices transparency, cite all existing rules, and known government provided data sources. . -->

### Hospitals

For now, check out the existing CMS [tools](https://www.medicare.gov/care-compare/?providerType=Hospital&redirect=true) for comparing hospital practices and ratings.

### Insurance Issuers

For now, check out the existing insurer [practice transparency data](https://www.cms.gov/CCIIO/Resources/Data-Resources/marketplace-puf) maintained by CMS (see e.g. the "transparency in coverage" puf files).

## Existence Transparency

A form of transparency that is implictly required to get the full utility of other forms of transparency is **existence transparency**. By this we
mean data that clearly indicates what the landscape of all existing hospitals, insurers and insurance plans are in the U.S.

There are many partial sources for this sort of data, but, as far as we know, no publicly accessible, centralized, complete, government maintained collection of such data. We'll try to maintain a csv of hospital metadata, insurance issuer metadata, and plan metadata.

Here are some examples of sources that can be used to generate partial or incomplete existence data:

<!-- TODO: Add similar resources for other states here, or elsewhere. -->

-  **Federal Marketplace Plan Data**
    - [Healthcare.gov researcher csvs](https://www.healthcare.gov/health-and-dental-plan-datasets-for-researchers-and-issuers/)
-  **State Marketplace Plan Data**:
    - California
        - [CA Department of Insurance](https://www.insurance.ca.gov/01-consumers/110-health/20-look/hcpcarriers.cfm)
- **Federal Marketplace Practice Data**
    - [CMS Public Use Files](https://www.cms.gov/CCIIO/Resources/Data-Resources/marketplace-puf)


### Hospitals

We maintain a flat file describing hospitals of which are we are aware in [`existence_transparency/hospitals/hospitals.csv`](./existence_transparency/hospitals/hospitals.csv). See [`existence_transparency/hospitals/README.md`](./existence_transparency/hospitals/README.md) for a description of the schema used in this file.

### Insurance Issuers

We maintain a flat file describing insurers we know about in [`existence_transparency/insurers/insurers.csv`](./existence_transparency/insurers/insurers.csv). See [`existence_transparency/insurers/README.md`](existence_transparency/insurers/README.md) for a description of the schema used in this file.

## Contributing

We welcome contributions from all, and feedback and discussion of any form.

Please see [`.github/CONTRIBUTING.md`](./.github/CONTRIBUTING.md) for more information about how to contribute to this project, or head to the github **Discussions** tab to join or start a conversation.


## Disclaimer

While we aim to provide accurate and up to date information, we provide this data with no warranty or guarantee of it's accuracy. We will do the best we can to verify the integrity of data provided here, but you should use it at your own risk, and avoid relying solely on it for critical matters. If you know
of any issues or mistakes in this data, please submit an issue or pull request, or contact the maintainers directly via the email listed below.

In particular, if you are a representative of a health insurer, hospital or other provider and seek to correct any of our existing data, or provide
information regarding missing data, your contribution is most welcome.

## Citation

If you find this data useful in your work, please consider mentioning it, providing a link, or citing it to help increase our exposure:

```latex
  @misc{persius2022transparency,
	author ={{Persius Contributors}},
	title={{CMS Transparency Rules Supplemental Data}},
	publisher = {GitHub},
	howpublished={\url{https://github.com/TPAFS/transparency-data}},
	year={2022},
}
```


## Contact
Feel free to reach out to community members, maintainers, or contributors via our discussions space with questions or comments.

Contact info@persius.org for private correspondence if that is needed for any reason.

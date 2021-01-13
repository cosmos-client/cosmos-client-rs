# ConnectionAssociatedWithTheRequestIdentifier

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> | client associated with this connection. | [optional]
**versions** | Option<[**Vec<crate::models::InlineResponse20080Versions>**](inline_response_200_80_versions.md)> | IBC version which can be utilised to determine encodings or protocols for channels or packets utilising this connection. | [optional]
**state** | Option<**String**> | current state of the connection end. | [optional][default to State_UNINITIALIZEDUNSPECIFIED]
**counterparty** | Option<[**crate::models::InlineResponse20080Counterparty**](inline_response_200_80_counterparty.md)> |  | [optional]
**delay_period** | Option<**String**> | delay period that must pass before a consensus state can be used for packet-verification NOTE: delay period logic is only implemented by some clients. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



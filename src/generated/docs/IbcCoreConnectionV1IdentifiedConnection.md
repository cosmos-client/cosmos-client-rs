# IbcCoreConnectionV1IdentifiedConnection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | connection identifier. | [optional]
**client_id** | Option<**String**> | client associated with this connection. | [optional]
**versions** | Option<[**Vec<crate::models::InlineResponse20080Versions>**](inline_response_200_80_versions.md)> |  | [optional]
**state** | Option<**String**> | current state of the connection end. | [optional][default to State_UNINITIALIZEDUNSPECIFIED]
**counterparty** | Option<[**crate::models::InlineResponse20080Counterparty**](inline_response_200_80_counterparty.md)> |  | [optional]
**delay_period** | Option<**String**> | delay period associated with this connection. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



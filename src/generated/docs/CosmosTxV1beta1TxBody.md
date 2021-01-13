# CosmosTxV1beta1TxBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**messages** | Option<[**Vec<crate::models::InlineResponseDefaultDetails>**](inline_response_default_details.md)> | messages is a list of messages to be executed. The required signers of those messages define the number and order of elements in AuthInfo's signer_infos and Tx's signatures. Each required signer address is added to the list only the first time it occurs.  By convention, the first required signer (usually from the first message) is referred to as the primary signer and pays the fee for the whole transaction. | [optional]
**memo** | Option<**String**> |  | [optional]
**timeout_height** | Option<**String**> |  | [optional]
**extension_options** | Option<[**Vec<crate::models::InlineResponseDefaultDetails>**](inline_response_default_details.md)> |  | [optional]
**non_critical_extension_options** | Option<[**Vec<crate::models::InlineResponseDefaultDetails>**](inline_response_default_details.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



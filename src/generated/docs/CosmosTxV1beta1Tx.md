# CosmosTxV1beta1Tx

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**body** | Option<[**crate::models::BodyIsTheProcessableContentOfTheTransaction**](body_is_the_processable_content_of_the_transaction.md)> |  | [optional]
**auth_info** | Option<[**crate::models::CosmosTxV1beta1AuthInfo**](cosmos.tx.v1beta1.AuthInfo.md)> |  | [optional]
**signatures** | Option<**Vec<String>**> | signatures is a list of signatures that matches the length and order of AuthInfo's signer_infos to allow connecting signature meta information like public key and signing mode by position. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



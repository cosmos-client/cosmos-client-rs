# CosmosTxV1beta1AuthInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**signer_infos** | Option<[**Vec<crate::models::CosmosTxV1beta1SignerInfo>**](cosmos.tx.v1beta1.SignerInfo.md)> | signer_infos defines the signing modes for the required signers. The number and order of elements must match the required signers from TxBody's messages. The first element is the primary signer and the one which pays the fee. | [optional]
**fee** | Option<[**crate::models::CosmosTxV1beta1AuthInfoFee**](cosmos_tx_v1beta1_AuthInfo_fee.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



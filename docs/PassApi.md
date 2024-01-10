# \PassApi

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_or_update_pass**](PassApi.md#create_or_update_pass) | **Post** /pass | This method creates or (if the pass id already exists) updates a pass, so you don&#39;t have to track ids and creation status of your passes.
[**delete_pass**](PassApi.md#delete_pass) | **Delete** /pass | Delete pass by pass id.
[**get_pass**](PassApi.md#get_pass) | **Get** /pass | Get pass information by pass id.
[**pass_list**](PassApi.md#pass_list) | **Get** /pass/list | Retrieve the list of active pass ids for a given pass type.
[**pass_sync**](PassApi.md#pass_sync) | **Post** /pass/sync | Send updates to all active passes for a given pass type.


# **create_or_update_pass**
> create_or_update_pass(pass_type_id, optional)
This method creates or (if the pass id already exists) updates a pass, so you don't have to track ids and creation status of your passes.

This method creates or (if the pass id already exists) updates a pass, so you don't have to track ids and creation status of your passes.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **pass_type_id** | [**Value**](.md)| your pass type id, for example P963493 (Urban Fitness) | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **pass_type_id** | [**Value**](.md)| your pass type id, for example P963493 (Urban Fitness) | 
 **pass_id** | [**Value**](.md)| id of the pass (provided by you when creating the pass or automatically set by passmeister) | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_pass**
> delete_pass(pass_type_id, pass_id)
Delete pass by pass id.

Delete pass by pass id.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **pass_type_id** | [**Value**](.md)| your pass type id, for example P963493 (Urban Fitness) | 
  **pass_id** | [**Value**](.md)| id of the pass | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_pass**
> get_pass(pass_type_id, pass_id)
Get pass information by pass id.

Get pass information by pass id.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **pass_type_id** | [**Value**](.md)| your pass type id, for example P963493 (Urban Fitness) | 
  **pass_id** | [**Value**](.md)| id of the pass | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **pass_list**
> pass_list(pass_type_id, optional)
Retrieve the list of active pass ids for a given pass type.

Retrieve the list of active pass ids for a given pass type.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **pass_type_id** | [**Value**](.md)| your pass type id, for example P963493 (Urban Fitness) | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **pass_type_id** | [**Value**](.md)| your pass type id, for example P963493 (Urban Fitness) | 
 **page** | [**Value**](.md)|  | 
 **limit** | [**Value**](.md)|  | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **pass_sync**
> pass_sync(pass_type_id)
Send updates to all active passes for a given pass type.

For example: you changed the pass type layout and now you want to update all installed passes. (The API call only confirms the scheduling of the updates, actual updating of passes on your customers devices can take a while.)

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **pass_type_id** | [**Value**](.md)| your pass type id, for example P963493 (Urban Fitness) | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# pdb

Private DB

## Goals

Protect db contents from root priv escalation

## Method

* run DAL in intel sgx enclave
* seal entries before inserting
* redis

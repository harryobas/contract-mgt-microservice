# contract-mgt-microservice
A proof of concept http/json microservice api for the management of contracts. Currently provide basic CRUDE (create, read, update, delete) features for contract resource.  

## Introduction
Managing contracts is a time-consuming task. They all have different terms, renewable periods,
termination windows and different ways to be terminated. Now imagine handling millions of
contracts: Such a scenario would obviously require a high performant service in terms of both throughput and response times. To this end, the contract-mgt-microservice has been implemented in Rust using the Rocket web framework with sqlite database backend.

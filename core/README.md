# Core

## Overview
At a high level, there are 3 main parts of the platform:
1. The smart contracts built on the ethereum blockchain. The recent change to proof of stake eliminated ~99% of its carbon footprint[^1]. Additionally, the use of a secure blockcahin will allow the network to be fully decentralized while ensuring public accessability to energy needs and production.
2. The DER API which will be used to interface with the smart contracts. This SDK will be used by prosumers to buy and sell energy to each other. It will also be used by the prosumer to interface with their solar panels, wind turbines, etc. and their smart meters to regulate electricity production and consumption.
3. The web application which will be used to visualize the network and the energy contracts. This will allow prosumers to see the energy needs and production of their neighbours and allow them to buy and sell energy to each other.

## `Energy Contracts`
### Technologies: `solidity`, `ethereum`
The smart contracts will be written in solidity and deployed on the ethereum mainnet. The smart contracts will be used to track the energy needs and production of prosumers and allow them to buy and sell energy to each other. The smart contracts will also be used to track the energy production of the DERs and allow the prosumers to buy and sell energy to the grid.

## `DER Engine`
### Technologies: `rust`, `go`, `raspberry pi`
The DER engine will be an API used to communicate with the smart meters to monitor production and consumption of electricity through the DERs. This will initially begin with solar panels as they produce low voltage and are currently deployed in resedential areas within the Greater Toronto Area. This will allow users to understand the amount of electricity they are able to produce and consume. This will be written in rust or go and will be deployed on a raspberry pi (initially).

## `Prosumer`
### Technologies: `NEXT`, `typescript`, `react`, `mapbox`
Prosumer will be the bring the above components to end users  by providing a user interface for the prosumers to interact with the network. This will allow them to see the energy needs and production of their neighbours using mapbox and allow them to buy and sell energy to each other. This will also allow them to see the energy production of their DERs and allow them to buy and sell energy to the grid in real time. The application will be akin to TORONTOVERSE[^2] but will also have a marketplace and production/consumption dashboard.


[^1]: https://consensys.net/blog/press-release/ethereum-blockchain-eliminates-99-99-of-its-carbon-footprint-overnight-after-a-successful-merge-according-to-new-report/
[^2]: https://torontoverse.com/
# Prosumer
Distributed energy resources are one of the key components of the energy transition. These resources include solar panels and wind turbines which allow for electricity to be produced near or at the point of use rather than a centralized power plant. This allows for consumers of energy (you and I) to become producers of energy. These consumers are called prosumers.

This project aims to provide a platform for prosumers to manage their energy production and consumption and distribute energy to neighbouring prosumers to combat the intermittency of renewable energy. The platform hopes to build on on the OpenADR 2.0b[^1] standard.

## Components

- `Prosumer`: A prosumer is a user of a DER. It can buy and sell energy to other prosumers. It can also produce energy from renewable sources.

 - `Energy Contracts`: A smart contract that allows prosumers to buy and sell energy to each other.
 - `Network`: A collection of prosumers that can buy and sell energy to each other. The network is a graph where each node is a prosumer and each edge is a *potential* contract.
 - `Prosumer`: A prosumer is a node in the network. It can buy and sell energy to other prosumers. It can also produce energy from renewable sources.
 - `Smart Meter`: A smart meter is a device that measures the energy consumption and production of a prosumer. It can also be used to control the energy consumption of a prosumer.
 - `Transmission Wires`: A transmission wire is a device that can be used to transfer energy from one prosumer to another. Additionally, these wires connect prosumers to the centralized grid to ensure 0 downtime.

## References
This project is heavily influenced by the OpenADR 2.0b standard, the book Energy for Future Presidents by Richard Muller and lessons from ECO314 Energy and the Environment at the University of Toronto.
[^1]: https://www.openadr.org/index.php?option=com_content&view=article&id=84
[^2]: Richard Muller, Energy for Future Presidents, Norton, 2012.

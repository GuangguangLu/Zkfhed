// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract PowerManagement {
    event WorkerRegistered(address indexed workerAddress, string machineResources);

    //Using mapping to maintain the worker registry
    struct Worker {
        string machineResources;
        bool isRegistered;
    }
    mapping(address => Worker) public registry;
    address[] public registeredWorkers;

   
    
    function registerWorker(string memory _machineResources) public {
        //Update registry
        registry[msg.sender] = Worker({
            machineResources: _machineResources,
            isRegistered: true
        });
        registeredWorkers.push(msg.sender);
        emit WorkerRegistered(msg.sender, _machineResources);
    }

    function getRegisteredWorkers() external view returns (address[] memory) {
        return registeredWorkers;
    }
}

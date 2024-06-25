// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract PowerManagement {


    event WorkerRegistered(address indexed workerAddress, string machineResources);


    function registerWorker(string memory _machineResources) public {

        emit WorkerRegistered(msg.sender, _machineResources);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract PowerManagement {

    struct Worker {
        address workerAddress;
        string name;
        string description;
        string machineResources;
        string rpcAddress;
        uint16 port;
        string publicKey;
    }

    mapping(address => Worker) public workers;
    address[] public workerAddresses;

    event WorkerRegistered(address indexed workerAddress, string name, string description);



    function registerWorker(
        string memory _name,
        string memory _description,
        string memory _machineResources,
        string memory _rpcAddress,
        uint16 _port,
        string memory _publicKey
    ) public {
        require(workers[msg.sender].workerAddress == address(0), "Worker already registered");

        workers[msg.sender] = Worker({
            workerAddress: msg.sender,
            name: _name,
            description: _description,
            machineResources: _machineResources,
            rpcAddress: _rpcAddress,
            port: _port,
            publicKey: _publicKey
        });

        workerAddresses.push(msg.sender);
        emit WorkerRegistered(msg.sender, _name, _description);
    }


    function getAllWorkers() public view returns (address[] memory) {
        return workerAddresses;
    }
}

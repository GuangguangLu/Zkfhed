// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract PowerManagement {

    // 定义事件来记录worker的注册
    event WorkerRegistered(address indexed workerAddress, string machineResources);

    // 注册Worker
    function registerWorker(string memory _machineResources) public {
        // 触发事件，记录worker的注册信息
        emit WorkerRegistered(msg.sender, _machineResources);
    }
}

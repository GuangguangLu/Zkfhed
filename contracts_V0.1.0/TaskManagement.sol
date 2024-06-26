// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./PowerManagement.sol";

contract TaskManagement {

    PowerManagement public powerManagement;

    struct Task {
        address requester;
        address[] dataOwnerAddresses;
        string[] requestCIDs;
        uint round;
        uint deadline;
    }

    event TaskAssigned(address indexed requester, string requestCID, address[] assignedWorkers);

    constructor(address _powerManagementAddress) {
        powerManagement = PowerManagement(_powerManagementAddress);
    }

    // Submit task
    function submitTask(
        address[] memory _dataOwnerAddresses,
        string[] memory _requestCIDs,
        uint _round,
        uint _deadline
    ) public {
        require(_dataOwnerAddresses.length == _requestCIDs.length, "Data owners and request CIDs length mismatch");

        // Get all registered workers
        address[] memory allWorkers = powerManagement.getAllWorkers();
        uint numWorkers = allWorkers.length;
        require(numWorkers >= 3, "Not enough registered workers");

        // Assign tasks
        for (uint i = 0; i < _requestCIDs.length; i++) {
            address[] memory assignedWorkers = new address[](3);
            assignedWorkers[0] = allWorkers[i % numWorkers];
            assignedWorkers[1] = allWorkers[(i + 1) % numWorkers];
            assignedWorkers[2] = allWorkers[(i + 2) % numWorkers];

            emit TaskAssigned(msg.sender, _requestCIDs[i], assignedWorkers);
        }
    }
}

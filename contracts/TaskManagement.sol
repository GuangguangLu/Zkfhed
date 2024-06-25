// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract TaskManagement {

    event TasksAssigned(address indexed requester, string requestCID1, address worker1, address worker2, string requestCID2, address worker3, address worker4);

    event TrainingResultSubmitted(
        address indexed worker,
        string trainingCID
    );

    constructor() {}

    function submitTask( 
        string memory _requestCID1,
        string memory _requestCID2,
        address worker1,
        address worker2,
        address worker3,
        address worker4
    ) public {
    
        emit TasksAssigned(
            msg.sender,
            _requestCID1,
            worker1,
            worker2,
            _requestCID2,
            worker3,
            worker4
        );
    }

    function submitResult(
        string memory _trainingCID
    ) public {
        emit TrainingResultSubmitted(
            msg.sender,
            _trainingCID
        );
    }
}

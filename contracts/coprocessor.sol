// SPDX-License-Identifier: MIT
pragma solidity >=0.8.4 <0.9.0;

contract Coprocessor {
    uint job_id = 0;
    address public coprocessor;

    constructor(address _coprocessor) {
        coprocessor = _coprocessor;
    }

    mapping(uint => string) public jobs;

    event NewJob(uint job_id);

    function newJob() public payable {
        job_id++;
        uint256 amount = address(this).balance;
        (bool success, ) = coprocessor.call{value: amount}("");
        require(success, "Failed to send Ether");
        emit NewJob(job_id);
    }

    function getResult(uint _job_id) public view returns (string memory) {
        return jobs[_job_id];
    }

    function callback(string calldata _result) public {
        require(
            msg.sender == coprocessor,
            "Only the coprocessor can call this function"
        );
        jobs[job_id] = _result;
    }
}

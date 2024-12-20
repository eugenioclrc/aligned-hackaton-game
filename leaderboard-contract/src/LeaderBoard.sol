// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {ERC1155} from "solady/tokens/ERC1155.sol";

contract LeaderBoard is ERC1155 {
    address private immutable _OWNER;

    struct Leader {
        address king; // @notice address(0) means no king
        uint96 steps; // @notice 0 means no steps
    }

    mapping(uint256 tokenId => uint256 total) public totalSupply;
    mapping(uint256 tokenId => Leader) public leaders;

    constructor() {
        // @notice the owner of this contract is the verifier contract
        _OWNER = msg.sender;
    }

    function name() public view returns (string memory) {
        return "Sokoban LeaderBoard";
    }

    function symbol() public view returns (string memory) {
        return "SOK";
    }

    function uri(uint256) public view override returns (string memory) {
        return
        "data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iaXNvLTg4NTktMSI/Pg0KPCEtLSBVcGxvYWRlZCB0bzogU1ZHIFJlcG8sIHd3dy5zdmdyZXBvLmNvbSwgR2VuZXJhdG9yOiBTVkcgUmVwbyBNaXhlciBUb29scyAtLT4NCjxzdmcgdmVyc2lvbj0iMS4xIiBpZD0iTGF5ZXJfMSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIiB4bWxuczp4bGluaz0iaHR0cDovL3d3dy53My5vcmcvMTk5OS94bGluayIgDQoJIHZpZXdCb3g9IjAgMCA1MTIuMDAxIDUxMi4wMDEiIHhtbDpzcGFjZT0icHJlc2VydmUiPg0KPHBvbHlnb24gc3R5bGU9ImZpbGw6IzNFQkQ5MTsiIHBvaW50cz0iMzg2LjU5MywxMC40NDcgMTI1LjQxLDEwLjQ0NyAxMjUuNDEsMTg2LjU1OCAxNzUuMzY1LDI2MC4xMzggMzQwLjgxNiwyNjAuMTM4IA0KCTM4Ni41OTMsMTg2LjU1OCAiLz4NCjxyZWN0IHg9IjE3Ny42NDciIHk9IjEwLjQ0NyIgc3R5bGU9ImZpbGw6I0VDRjBGMTsiIHdpZHRoPSIxNTYuNzA5IiBoZWlnaHQ9IjI0OS42OSIvPg0KPHJlY3QgeD0iMjE5LjQzNiIgeT0iMTAuNTMxIiBzdHlsZT0iZmlsbDojM0VCRDkxOyIgd2lkdGg9IjczLjEzMSIgaGVpZ2h0PSIyNDkuNTIzIi8+DQo8ZWxsaXBzZSBzdHlsZT0iZmlsbDojRjhDNjYwOyIgY3g9IjI1Ni4wMDIiIGN5PSIzNjMuMjIxIiByeD0iMTM4LjQyNyIgcnk9IjEzOC4zMzMiLz4NCjxwYXRoIHN0eWxlPSJmaWxsOiNFQ0YwRjE7IiBkPSJNMjU2LjAwMiw0NTkuNzkzYy01My4yODYsMC05Ni42MzctNDMuMzIyLTk2LjYzNy05Ni41NzJzNDMuMzUxLTk2LjU3Miw5Ni42MzctOTYuNTcyDQoJczk2LjYzNyw0My4zMjIsOTYuNjM3LDk2LjU3MlMzMDkuMjg4LDQ1OS43OTMsMjU2LjAwMiw0NTkuNzkzeiIvPg0KPGc+DQoJPHBhdGggc3R5bGU9ImZpbGw6IzIzMUYyMDsiIGQ9Ik0zNTYuOTI0LDI1My45NTJsMzguNTM2LTYxLjg3YzEuMDMyLTEuNjU3LDEuNTgtMy41NywxLjU4LTUuNTIzVjEwLjM2NQ0KCQlDMzk3LjA0LDQuNTk1LDM5Mi4zNjMsMCwzODYuNTkzLDBIMTI1LjQxYy01Ljc3LDAtMTAuNDQ3LDQuNTk1LTEwLjQ0NywxMC4zNjV2MTc2LjE5NGMwLDIuMDk1LDAuNjMsNC4xNCwxLjgwNiw1Ljg3M2w0MC40OCw1OS41NTYNCgkJYy0zMC43MjUsMjcuMjc0LTUwLjEyMyw2Ny4wMy01MC4xMjMsMTExLjIzM2MwLDgyLjAzNyw2Ni43ODQsMTQ4Ljc4LDE0OC44NzQsMTQ4Ljc4czE0OC44NzQtNjYuNzQzLDE0OC44NzQtMTQ4Ljc4DQoJCUM0MDQuODc2LDMyMC4wNjgsMzg2LjM5LDI4MS4xNTIsMzU2LjkyNCwyNTMuOTUyeiBNMzc2LjE0NSwxODMuNTcybC0zMS4zNDIsNTAuMzJWMjAuODk1aDMxLjM0MlYxODMuNTcyeiBNMzIzLjkwOSwyMzAuODQxDQoJCWMtNi42ODctMy40NC0xMy42NjktNi4zODQtMjAuODk1LTguNzkzVjIwLjg5NWgyMC44OTVWMjMwLjg0MXogTTI4Mi4xMiwyMC44OTV2MTk1Ljg0Yy04LjQ4Mi0xLjUwNy0xNy4yMS0yLjI5NC0yNi4xMTgtMi4yOTQNCgkJcy0xNy42MzYsMC43ODktMjYuMTE4LDIuMjk0VjIwLjg5NUgyODIuMTJ6IE0yMDguOTg5LDIwLjg5NXYyMDEuMTUzYy03LjIyNSwyLjQwOS0xNC4yMDcsNS4zNTMtMjAuODk1LDguNzkzVjIwLjg5NUgyMDguOTg5eg0KCQkgTTEzNS44NTgsMjAuODk1SDE2Ny4ydjIwOC41NmwtMzEuMzQyLTQ2LjExMVYyMC44OTV6IE0yNTYuMDAyLDQ5MS4xMDVjLTcwLjU2OCwwLTEyNy45NzktNTcuMzY5LTEyNy45NzktMTI3Ljg4NQ0KCQlzNTcuNDExLTEyNy44ODUsMTI3Ljk3OS0xMjcuODg1czEyNy45NzksNTcuMzY5LDEyNy45NzksMTI3Ljg4NVMzMjYuNTcsNDkxLjEwNSwyNTYuMDAyLDQ5MS4xMDV6Ii8+DQoJPHBhdGggc3R5bGU9ImZpbGw6IzIzMUYyMDsiIGQ9Ik0yNTYuMDAyLDI1Ni4yMDFjLTU5LjA0NywwLTEwNy4wODUsNDguMDA4LTEwNy4wODUsMTA3LjAyczQ4LjAzOCwxMDcuMDIsMTA3LjA4NSwxMDcuMDINCgkJczEwNy4wODUtNDguMDA4LDEwNy4wODUtMTA3LjAyUzMxNS4wNDgsMjU2LjIwMSwyNTYuMDAyLDI1Ni4yMDF6IE0yNTYuMDAyLDQ0OS4zNDVjLTQ3LjUyNiwwLTg2LjE5LTM4LjYzNS04Ni4xOS04Ni4xMjUNCgkJczM4LjY2NC04Ni4xMjUsODYuMTktODYuMTI1czg2LjE5LDM4LjYzNSw4Ni4xOSw4Ni4xMjVTMzAzLjUyNiw0NDkuMzQ1LDI1Ni4wMDIsNDQ5LjM0NXoiLz4NCjwvZz4NCjwvc3ZnPg==";
    }

    function mint(uint256 tokenId, address leader, uint256 steps) external {
        require(msg.sender == _OWNER, "!OWNER");
        Leader storage currentLeader = leaders[tokenId];

        if (currentLeader.king != address(0)) {
            require(currentLeader.steps > steps, "Cant beat leader");
            currentLeader.king = leader;
            currentLeader.steps = uint96(steps);
        } else {
            currentLeader.king = leader;
            currentLeader.steps = uint96(steps);
        }

        _mint(leader, tokenId, totalSupply[tokenId]++, "");
    }
}

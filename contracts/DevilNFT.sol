// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/**
 * @title DevilNFT
 * @notice ERC-721-compatible NFT for DevilChain Network
 */
contract DevilNFT {
    string public name = "DevilChain NFT";
    string public symbol = "DVLNFT";
    uint256 public totalSupply;

    mapping(uint256 => address) public ownerOf;
    mapping(address => uint256) public balanceOf;
    mapping(uint256 => string) public tokenURI;
    mapping(uint256 => address) public getApproved;
    mapping(address => mapping(address => bool)) public isApprovedForAll;

    event Transfer(address indexed from, address indexed to, uint256 indexed tokenId);
    event Approval(address indexed owner, address indexed approved, uint256 indexed tokenId);

    function mint(address to, string calldata uri) external returns (uint256) {
        totalSupply++;
        uint256 tokenId = totalSupply;
        ownerOf[tokenId] = to;
        balanceOf[to]++;
        tokenURI[tokenId] = uri;
        emit Transfer(address(0), to, tokenId);
        return tokenId;
    }

    function transferFrom(address from, address to, uint256 tokenId) external {
        require(ownerOf[tokenId] == from, "Not owner");
        require(
            msg.sender == from ||
            msg.sender == getApproved[tokenId] ||
            isApprovedForAll[from][msg.sender],
            "Not authorized"
        );
        ownerOf[tokenId] = to;
        balanceOf[from]--;
        balanceOf[to]++;
        getApproved[tokenId] = address(0);
        emit Transfer(from, to, tokenId);
    }
}

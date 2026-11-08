// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./BridgeBaseTest.t.sol";
import "../contracts/utils/BridgeUtils.sol";

contract BridgeCommitteeTest is BridgeBaseTest {
    // This function is called before each unit test
    function setUp() public {
        setUpBridgeTest();
    }

    function testBridgeCommitteeInitialization() public view {
        assertEq(committee.committeeStake(committeeMemberA), 1000);
        assertEq(committee.committeeStake(committeeMemberB), 1000);
        assertEq(committee.committeeStake(committeeMemberC), 1000);
        assertEq(committee.committeeStake(committeeMemberD), 2002);
        assertEq(committee.committeeStake(committeeMemberE), 4998);
        // Assert that the total stake is 10,000
        assertEq(
            committee.committeeStake(committeeMemberA) + committee.committeeStake(committeeMemberB)
                + committee.committeeStake(committeeMemberC)
                + committee.committeeStake(committeeMemberD)
                + committee.committeeStake(committeeMemberE),
            10000
        );
        // Check that the blocklist and nonces are initialized to zero
        assertEq(committee.blocklist(address(committeeMemberA)), false);
        assertEq(committee.blocklist(address(committeeMemberB)), false);
        assertEq(committee.blocklist(address(committeeMemberC)), false);
        assertEq(committee.blocklist(address(committeeMemberD)), false);
        assertEq(committee.blocklist(address(committeeMemberE)), false);
        assertEq(committee.nonces(0), 0);
        assertEq(committee.nonces(1), 0);
        assertEq(committee.nonces(2), 0);
        assertEq(committee.nonces(3), 0);
        assertEq(committee.nonces(4), 0);
    }

    function testBridgeCommitteeInitializationLength() public {
        address[] memory _committeeMembers = new address[](256);

        for (uint160 i = 0; i < 256; i++) {
            _committeeMembers[i] = address(i);
        }

        address _committee = Upgrades.deployUUPSProxy("BridgeCommittee.sol", "", opts);

        vm.expectRevert(bytes("BridgeCommittee: Committee length must be less than 256"));
        BridgeCommittee(_committee).initialize(
            _committeeMembers, new uint16[](256), minStakeRequired
        );
    }

    function testBridgeCommitteeInitializeConfig() public {
        vm.expectRevert(bytes("BridgeCommittee: Config already initialized"));
        // Initialize the committee with the config contract
        committee.initializeConfig(address(101));
    }

    function testBridgeFailInitialization() public {
        // Test fail initialize: Committee Duplicate Committee Member
        address[] memory _committeeDuplicateCommitteeMember = new address[](5);
        _committeeDuplicateCommitteeMember[0] = committeeMemberA;
        _committeeDuplicateCommitteeMember[1] = committeeMemberB;
        _committeeDuplicateCommitteeMember[2] = committeeMemberC;
        _committeeDuplicateCommitteeMember[3] = committeeMemberD;
        _committeeDuplicateCommitteeMember[4] = committeeMemberA;

        uint16[] memory _stakeDuplicateCommitteeMember = new uint16[](5);
        _stakeDuplicateCommitteeMember[0] = 1000;
        _stakeDuplicateCommitteeMember[1] = 1000;
        _stakeDuplicateCommitteeMember[2] = 1000;
        _stakeDuplicateCommitteeMember[3] = 2002;
        _stakeDuplicateCommitteeMember[4] = 1000;

        address _committee = Upgrades.deployUUPSProxy("BridgeCommittee.sol", "", opts);

        committee = BridgeCommittee(_committee);

        vm.expectRevert(bytes("BridgeCommittee: Duplicate committee member"));

        committee.initialize(
            _committeeDuplicateCommitteeMember, _stakeDuplicateCommitteeMember, minStakeRequired
        );

        address[] memory _committeeNotSameLength = new address[](5);
        _committeeNotSameLength[0] = committeeMemberA;
        _committeeNotSameLength[1] = committeeMemberB;
        _committeeNotSameLength[2] = committeeMemberC;
        _committeeNotSameLength[3] = committeeMemberD;
        _committeeNotSameLength[4] = committeeMemberE;

        uint16[] memory _stakeNotSameLength = new uint16[](4);
        _stakeNotSameLength[0] = 1000;
        _stakeNotSameLength[1] = 1000;
        _stakeNotSameLength[2] = 1000;
        _stakeNotSameLength[3] = 2002;

        vm.expectRevert(
            bytes("BridgeCommittee: Committee and stake arrays must be of the same length")
        );

        committee.initialize(_committeeNotSameLength, _stakeNotSameLength, minStakeRequired);
    }

    function testVerifySignaturesWithValidSignatures() public view {
        // Create a message
        BridgeUtils.Message memory message = BridgeUtils.Message({
            messageType: BridgeUtils.TOKEN_TRANSFER,
            version: 1,
            nonce: 1,
            chainID: chainID,
            payload: "0x0"
        });

        bytes memory messageBytes = BridgeUtils.encodeMessage(message);

        bytes32 messageHash = keccak256(messageBytes);

        bytes[] memory signatures = new bytes[](4);

        // Create signatures from A - D
        signatures[0] = getSignature(messageHash, committeeMemberPkA);
        signatures[1] = getSignature(messageHash, committeeMemberPkB);
        signatures[2] = getSignature(messageHash, committeeMemberPkC);
        signatures[3] = getSignature(messageHash, committeeMemberPkD);

        // Call the verifySignatures function and it would not revert
        committee.verifySignatures(signatures, message);
    }

    function testVerifySignaturesWithInvalidSignatures() public {
        // Create a message
        BridgeUtils.Message memory message = BridgeUtils.Message({
            messageType: BridgeUtils.TOKEN_TRANSFER,
            version: 1,
            nonce: 1,
            chainID: chainID,
            payload: "0x0"
        });

        bytes memory messageBytes = BridgeUtils.encodeMessage(message);

        bytes32 messageHash = keccak256(messageBytes);

        bytes[] memory signatures = new bytes[](3);

        // Create signatures from A - D
        signatures[0] = getSignature(messageHash, committeeMemberPkA);
        signatures[1] = getSignature(messageHash, committeeMemberPkB);
        signatures[2] = getSignature(messageHash, committeeMemberPkC);

        // Call the verifySignatures function and expect it to revert
        vm.expectRevert(bytes("BridgeCommittee: Insufficient stake amount"));
        committee.verifySignatures(signatures, message);
    }

    function testVerifySignaturesDuplicateSignature() public {
        // Create a message
        BridgeUtils.Message memory message = BridgeUtils.Message({
            messageType: BridgeUtils.TOKEN_TRANSFER,
            version: 1,
            nonce: 1,
            chainID: chainID,
            payload: "0x0"
        });

        bytes memory messageBytes = BridgeUtils.encodeMessage(message);
        bytes32 messageHash = keccak256(messageBytes);

        bytes[] memory signatures = new bytes[](4);

        // Create signatures from A - C
        signatures[0] = getSignature(messageHash, committeeMemberPkA);
        signatures[1] = getSignature(messageHash, committeeMemberPkA);
        signatures[2] = getSignature(messageHash, committeeMemberPkB);
        signatures[3] = getSignature(messageHash, committeeMemberPkC);

        // Call the verifySignatures function and expect it to revert
        vm.expectRevert(bytes("BridgeCommittee: Duplicate signature provided"));
        committee.verifySignatures(signatures, message);
    }

    function testRevertUpdateBlocklistWithSignaturesInvalidNonce() public {
        // create payload
        address[] memory _blocklist = new address[](1);
        _blocklist[0] = committeeMemberA;
        bytes memory payload = hex"0001";
        payload = abi.encodePacked(payload, committeeMemberA);

        // Create a message with wrong nonce
        BridgeUtils.Message memory messageWrongNonce = BridgeUtils.Message({
            messageType: BridgeUtils.BLOCKLIST,
            version: 1,
            nonce: 1,
            chainID: chainID,
            payload: payload
        });
        bytes memory messageBytes = BridgeUtils.encodeMessage(messageWrongNonce);
        bytes32 messageHash = keccak256(messageBytes);
        bytes[] memory signatures = new bytes[](4);

        // Create signatures from A - D
        signatures[0] = getSignature(messageHash, committeeMemberPkA);
        signatures[1] = getSignature(messageHash, committeeMemberPkB);
        signatures[2] = getSignature(messageHash, committeeMemberPkC);
        signatures[3] = getSignature(messageHash, committeeMemberPkD);
        vm.expectRevert(bytes("MessageVerifier: Invalid nonce"));
        committee.updateBlocklistWithSignatures(signatures, messageWrongNonce);
    }

    function testUpdateBlocklistWithSignaturesMessageDoesNotMatchType() public {
        // create payload
        address[] memory _blocklist = new address[](1);
        _blocklist[0] = committeeMemberA;
        bytes memory payload = abi.encode(uint8(0), _blocklist);

        // Create a message with wrong messageType
        BridgeUtils.Message memory messageWrongMessageType = BridgeUtils.Message({
            messageType: BridgeUtils.TOKEN_TRANSFER,
            version: 1,
            nonce: 0,
            chainID: chainID,
            payload: payload
        });
        bytes memory messageBytes = BridgeUtils.encodeMessage(messageWrongMessageType);
        bytes32 messageHash = keccak256(messageBytes);
        bytes[] memory signatures = new bytes[](4);

        // Create signatures from A - D
        signatures[0] = getSignature(messageHash, committeeMemberPkA);
        signatures[1] = getSignature(messageHash, committeeMemberPkB);
        signatures[2] = getSignature(messageHash, committeeMemberPkC);
        signatures[3] = getSignature(messageHash, committeeMemberPkD);
        vm.expectRevert(bytes("MessageVerifier: message does not match type"));
        committee.updateBlocklistWithSignatures(signatures, messageWrongMessageType);
    }

    function testRevertUpdateBlocklistWithSignaturesInvalidSignatures() public {
        // create payload
        address[] memory _blocklist = new address[](1);
        _blocklist[0] = committeeMemberA;
        bytes memory payload = hex"0001";
        payload = abi.encodePacked(payload, committeeMemberA);

        // Create a message
        BridgeUtils.Message memory message = BridgeUtils.Message({
            messageType: BridgeUtils.BLOCKLIST,
            version: 1,
            nonce: 0,
            chainID: chainID,
            payload: payload
        });
        bytes memory messageBytes = BridgeUtils.encodeMessage(message);
        bytes32 messageHash = keccak256(messageBytes);
        bytes[] memory signatures = new bytes[](1);

        // Create signatures from A
        signatures[0] = getSignature(messageHash, committeeMemberPkA);
        vm.expectRevert(bytes("BridgeCommittee: Insufficient stake amount"));
        committee.updateBlocklistWithSignatures(signatures, message);
    }

    function testAddToBlocklist() public {
        // create payload
        address[] memory _blocklist = new address[](1);
        _blocklist[0] = committeeMemberA;
        bytes memory payload = hex"0001";
        payload = abi.encodePacked(payload, committeeMemberA);

        // Create a message
        BridgeUtils.Message memory message = BridgeUtils.Message({
            messageType: BridgeUtils.BLOCKLIST,
            version: 1,
            nonce: 0,
            chainID: chainID,
            payload: payload
        });

        bytes memory messageBytes = BridgeUtils.encodeMessage(message);
        bytes32 messageHash = keccak256(messageBytes);
        bytes[] memory signatures = new bytes[](4);

        // Create signatures from A - D
        signatures[0] = getSignature(messageHash, committeeMemberPkA);
        signatures[1] = getSignature(messageHash, committeeMemberPkB);
        signatures[2] = getSignature(messageHash, committeeMemberPkC);
        signatures[3] = getSignature(messageHash, committeeMemberPkD);

        committee.updateBlocklistWithSignatures(signatures, message);

        assertTrue(committee.blocklist(committeeMemberA));

        // update message
        message.nonce = 1;
        // reconstruct signatures
        messageBytes = BridgeUtils.encodeMessage(message);
        messageHash = keccak256(messageBytes);
        signatures[0] = getSignature(messageHash, committeeMemberPkA);
        signatures[1] = getSignature(messageHash, committeeMemberPkB);
        signatures[2] = getSignature(messageHash, committeeMemberPkC);
        signatures[3] = getSignature(messageHash, committeeMemberPkD);
        // verify CommitteeMemberA's signature is no longer valid
        vm.expectRevert(bytes("BridgeCommittee: Signer is blocklisted"));
        // re-verify signatures
        committee.verifySignatures(signatures, message);
    }

    function testSignerNotCommitteeMember() public {
        // create payload
        bytes memory payload = abi.encode(committeeMemberA);

        // Create a message
        BridgeUtils.Message memory message = BridgeUtils.Message({
            messageType: BridgeUtils.UPGRADE,
            version: 1,
            nonce: 0,
            chainID: chainID,
            payload: payload
        });

        bytes memory messageBytes = BridgeUtils.encodeMessage(message);
        bytes32 messageHash = keccak256(messageBytes);
        bytes[] memory signatures = new bytes[](4);

        (, uint256 committeeMemberPkF) = makeAddrAndKey("f");

        // Create signatures from A - D, and F
        signatures[0] = getSignature(messageHash, committeeMemberPkA);
        signatures[1] = getSignature(messageHash, committeeMemberPkB);
        signatures[2] = getSignature(messageHash, committeeMemberPkC);
        signatures[3] = getSignature(messageHash, committeeMemberPkF);

        vm.expectRevert(bytes("BridgeCommittee: Signer has no stake"));
        committee.verifySignatures(signatures, message);
    }

    function testRemoveFromBlocklist() public {
        testAddToBlocklist();

        // create payload
        address[] memory _blocklist = new address[](1);
        _blocklist[0] = committeeMemberA;
        bytes memory payload = hex"0101";
        payload = abi.encodePacked(payload, committeeMemberA);

        // Create a message
        BridgeUtils.Message memory message = BridgeUtils.Message({
            messageType: BridgeUtils.BLOCKLIST,
            version: 1,
            nonce: 1,
            chainID: chainID,
            payload: payload
        });

        bytes memory messageBytes = BridgeUtils.encodeMessage(message);
        bytes32 messageHash = keccak256(messageBytes);
        bytes[] memory signatures = new bytes[](4);

        // Create signatures from B - E
        signatures[0] = getSignature(messageHash, committeeMemberPkB);
        signatures[1] = getSignature(messageHash, committeeMemberPkC);
        signatures[2] = getSignature(messageHash, committeeMemberPkD);
        signatures[3] = getSignature(messageHash, committeeMemberPkE);

        committee.updateBlocklistWithSignatures(signatures, message);

        // verify CommitteeMemberA is no longer blocklisted
        assertFalse(committee.blocklist(committeeMemberA));
    }

    // An e2e update committee blocklist regression test covering message ser/de
    function testUpdateCommitteeBlocklistRegressionTest() public pure {
        bytes memory payload =
            hex"010268b43fd906c0b8f024a18c56e06744f7c6157c65acaef39832cb995c4e049437a3e2ec6a7bad1ab5";
        // Create blocklist message
        BridgeUtils.Message memory message = BridgeUtils.Message({
            messageType: BridgeUtils.BLOCKLIST,
            version: 1,
            nonce: 68,
            chainID: 2,
            payload: payload
        });
        bytes memory encodedMessage = BridgeUtils.encodeMessage(message);
        bytes memory expectedEncodedMessage =
            hex"48414e45554c5f4252494447455f4d4553534147450101000000000000004402010268b43fd906c0b8f024a18c56e06744f7c6157c65acaef39832cb995c4e049437a3e2ec6a7bad1ab5";

        assertEq(encodedMessage, expectedEncodedMessage);
    }

    // An e2e update committee blocklist regression test covering message ser/de and signature verification
    function testUpdateCommitteeBlocklistRegressionTestWithSignatures() public {
        address[] memory _committeeList = new address[](4);
        uint16[] memory _stake = new uint16[](4);
        uint8 chainID = 11;
        _committeeList[0] = 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266;
        _committeeList[1] = 0x70997970C51812dc3A010C7d01b50e0d17dc79C8;
        _committeeList[2] = 0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC;
        _committeeList[3] = 0x90F79bf6EB2c4f870365E785982E1f101E93b906;
        _stake[0] = 2500;
        _stake[1] = 2500;
        _stake[2] = 2500;
        _stake[3] = 2500;
        address _committee = Upgrades.deployUUPSProxy(
            "BridgeCommittee.sol",
            abi.encodeCall(BridgeCommittee.initialize, (_committeeList, _stake, minStakeRequired)),
            opts
        );
        committee = BridgeCommittee(_committee);

        address _config = Upgrades.deployUUPSProxy(
            "BridgeConfig.sol",
            abi.encodeCall(
                BridgeConfig.initialize,
                (
                    address(committee),
                    chainID,
                    supportedTokens,
                    tokenPrices,
                    tokenIds,
                    haneulDecimals,
                    supportedChains
                )
            ),
            opts
        );

        committee.initializeConfig(_config);

        assertEq(committee.blocklist(0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266), false);

        // blocklist 1 member addr1 ("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266")
        bytes memory payload = hex"0001f39fd6e51aad88f6f4ce6ab8827279cfffb92266";
        BridgeUtils.Message memory message = BridgeUtils.Message({
            messageType: BridgeUtils.BLOCKLIST,
            version: 1,
            nonce: 0,
            chainID: chainID,
            payload: payload
        });
        bytes memory encodedMessage = BridgeUtils.encodeMessage(message);
        bytes memory expectedEncodedMessage =
            hex"48414e45554c5f4252494447455f4d455353414745010100000000000000000b0001f39fd6e51aad88f6f4ce6ab8827279cfffb92266";

        assertEq(encodedMessage, expectedEncodedMessage);

        bytes[] memory signatures = new bytes[](3);

        signatures[0] =
            hex"7ce744b7c7124ecdd3b8eca648f8b1967e6510ac6fb40f05767433a9764e99260bbb8a214c7133407a47a617cc663036ee72ffc84d81ad950e2253f6dccd704501";
        signatures[1] =
            hex"34c8d98b0e489c3f7fb3daf6aa30bfccb3d1ecc4e405c73d0382ffcbb9f283ec23e46cb7789f346c7401ce48cec62eacaf3155ddc59c905cbb658c1dbe52432001";
        signatures[2] =
            hex"85777a40bb70f998cb6ae3dbd66d5efa85f5aa018009650fdc0d3d6a7a6ef0381bb069124bb4e2676c422571f6fe2e60f044bbc9a02274687aaf10b7eb6a1c6c01";

        committee.verifySignatures(signatures, message);

        committee.updateBlocklistWithSignatures(signatures, message);

        assertEq(committee.blocklist(0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266), true);

        // unblocklist 1 member addr1 ("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266")
        payload = hex"0101f39fd6e51aad88f6f4ce6ab8827279cfffb92266";
        message = BridgeUtils.Message({
            messageType: BridgeUtils.BLOCKLIST,
            version: 1,
            nonce: 1,
            chainID: chainID,
            payload: payload
        });
        encodedMessage = BridgeUtils.encodeMessage(message);
        expectedEncodedMessage =
            hex"48414e45554c5f4252494447455f4d455353414745010100000000000000010b0101f39fd6e51aad88f6f4ce6ab8827279cfffb92266";

        assertEq(encodedMessage, expectedEncodedMessage);

        signatures = new bytes[](3);

        // Note sig[0] is from blocklisetd validator, and it does not count.
        signatures[0] =
            hex"ffe8fcbebbb81b01cffe70766276a55aa1d03e39cc3b2366ef0a4c8df13991b739675eada6e47b08e64dee3d0699c7263bb6cbfe74a11df5c965d6b2d7ada8e600";
        signatures[1] =
            hex"3ead3b0c928f9615fa457f66d11b956bc990e008df15d62e8f4849f6066d45f000fe4606fc2f97a122ec37e21e7535464eeccd8cf48582cf1a803c28792ec02500";
        signatures[2] =
            hex"f1495e3a07b6cfc40e841062404fca1a150b7d3281423565e0546214f917929516ed487d8da6c4c3c535363cdda1b949ecaac531cb9d8542396e35a2d483cdcc00";

        vm.expectRevert(bytes("BridgeCommittee: Signer is blocklisted"));
        committee.verifySignatures(signatures, message);

        // use sig from a unblocklisted validator
        signatures[0] =
            hex"e7531e483de908b5e29a55871e9ba66cac0eb6d76a1e7818bc55d1ec54f331d17a39a2b8762f75a7383e03bdba3d3e729da021f3328fc9582378c1aef48439cb00";
        committee.verifySignatures(signatures, message);
        committee.updateBlocklistWithSignatures(signatures, message);
        assertEq(committee.blocklist(0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266), false);
    }
}

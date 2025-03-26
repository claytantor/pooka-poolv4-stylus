// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {Test} from "forge-std/Test.sol";
import {CurrencyLibrary, Currency} from "../src/types/Currency.sol";
import {MockERC6909Claims} from "../src/test/MockERC6909Claims.sol";

contract ERC6909ClaimsTest is Test {
    using CurrencyLibrary for Currency;

    MockERC6909Claims token;

    function setUp() public {
        token = new MockERC6909Claims();
    }

    function test_burnFrom_withApproval(address sender, uint256 id, uint256 mintAmount, uint256 transferAmount)
        public
    {
        token.mint(sender, id, mintAmount);

        vm.prank(sender);
        token.approve(address(this), id, mintAmount);

        if (transferAmount > mintAmount) {
            vm.expectRevert();
        }
        token.burnFrom(sender, id, transferAmount);

        if (transferAmount <= mintAmount) {
            if (mintAmount == type(uint256).max) {
                assertEq(token.allowance(sender, address(this), id), type(uint256).max);
            } else {
                assertEq(token.allowance(sender, address(this), id), mintAmount - transferAmount);
            }
            assertEq(token.balanceOf(sender, id), mintAmount - transferAmount);
        }
    }

    function test_burnFrom_revertsWithNoApproval() public {
        token.mint(address(this), 1337, 100);

        vm.prank(address(0xBEEF));
        vm.expectRevert();
        token.burnFrom(address(this), 1337, 100);
    }

    /// ---- Tests copied from solmate ---- ///

    function testMint() public {
        token.mint(address(0xBEEF), 1337, 100);

        assertEq(token.balanceOf(address(0xBEEF), 1337), 100);
    }

    function testBurn() public {
        token.mint(address(0xBEEF), 1337, 100);
        vm.prank(address(0xBEEF));
        token.burn(1337, 70);

        assertEq(token.balanceOf(address(0xBEEF), 1337), 30);
    }

    function testSetOperator() public {
        token.setOperator(address(0xBEEF), true);

        assertTrue(token.isOperator(address(this), address(0xBEEF)));
    }

    function testApprove() public {
        token.approve(address(0xBEEF), 1337, 100);

        assertEq(token.allowance(address(this), address(0xBEEF), 1337), 100);
    }

    function testTransfer() public {
        address sender = address(0xABCD);

        token.mint(sender, 1337, 100);

        vm.prank(sender);
        token.transfer(address(0xBEEF), 1337, 70);

        assertEq(token.balanceOf(sender, 1337), 30);
        assertEq(token.balanceOf(address(0xBEEF), 1337), 70);
    }

    function testTransferFromWithApproval() public {
        address sender = address(0xABCD);
        address receiver = address(0xBEEF);

        token.mint(sender, 1337, 100);

        vm.prank(sender);
        token.approve(address(this), 1337, 100);

        token.transferFrom(sender, receiver, 1337, 70);

        assertEq(token.allowance(sender, address(this), 1337), 30);
        assertEq(token.balanceOf(sender, 1337), 30);
        assertEq(token.balanceOf(receiver, 1337), 70);
    }

    function testTransferFromWithInfiniteApproval() public {
        address sender = address(0xABCD);
        address receiver = address(0xBEEF);

        token.mint(sender, 1337, 100);

        vm.prank(sender);
        token.approve(address(this), 1337, type(uint256).max);

        token.transferFrom(sender, receiver, 1337, 70);

        assertEq(token.allowance(sender, address(this), 1337), type(uint256).max);
        assertEq(token.balanceOf(sender, 1337), 30);
        assertEq(token.balanceOf(receiver, 1337), 70);
    }

    function testTransferFromAsOperator() public {
        address sender = address(0xABCD);
        address receiver = address(0xBEEF);

        token.mint(sender, 1337, 100);

        vm.prank(sender);
        token.setOperator(address(this), true);

        token.transferFrom(sender, receiver, 1337, 70);

        assertEq(token.balanceOf(sender, 1337), 30);
        assertEq(token.balanceOf(receiver, 1337), 70);
    }

    function testFailMintBalanceOverflow() public {
        token.mint(address(0xDEAD), 1337, type(uint256).max);
        token.mint(address(0xDEAD), 1337, 1);
    }

    function testFailTransferBalanceUnderflow() public {
        address sender = address(0xABCD);
        address receiver = address(0xBEEF);

        vm.prank(sender);
        token.transferFrom(sender, receiver, 1337, 1);
    }

    function testFailTransferBalanceOverflow() public {
        address sender = address(0xABCD);
        address receiver = address(0xBEEF);

        token.mint(sender, 1337, type(uint256).max);

        vm.prank(sender);
        token.transferFrom(sender, receiver, 1337, type(uint256).max);

        token.mint(sender, 1337, 1);

        vm.prank(sender);
        token.transferFrom(sender, receiver, 1337, 1);
    }

    function testFailTransferFromBalanceUnderflow() public {
        address sender = address(0xABCD);
        address receiver = address(0xBEEF);

        vm.prank(sender);
        token.transferFrom(sender, receiver, 1337, 1);
    }

    function testFailTransferFromBalanceOverflow() public {
        address sender = address(0xABCD);
        address receiver = address(0xBEEF);

        token.mint(sender, 1337, type(uint256).max);

        vm.prank(sender);
        token.transferFrom(sender, receiver, 1337, type(uint256).max);

        token.mint(sender, 1337, 1);

        vm.prank(sender);
        token.transferFrom(sender, receiver, 1337, 1);
    }

    function testFailTransferFromNotAuthorized() public {
        address sender = address(0xABCD);
        address receiver = address(0xBEEF);

        token.mint(sender, 1337, 100);

        token.transferFrom(sender, receiver, 1337, 100);
    }

    function testMint(address receiver, uint256 id, uint256 amount) public {
        token.mint(receiver, id, amount);

        assertEq(token.balanceOf(receiver, id), amount);
    }

    function testBurn(address sender, uint256 id, uint256 amount) public {
        token.mint(sender, id, amount);
        vm.prank(sender);
        token.burn(id, amount);

        assertEq(token.balanceOf(sender, id), 0);
    }

    function testSetOperator(address operator, bool approved) public {
        token.setOperator(operator, approved);

        assertEq(token.isOperator(address(this), operator), approved);
    }

    function testApprove(address spender, uint256 id, uint256 amount) public {
        token.approve(spender, id, amount);

        assertEq(token.allowance(address(this), spender, id), amount);
    }

    function testTransfer(address sender, address receiver, uint256 id, uint256 mintAmount, uint256 transferAmount)
        public
    {
        transferAmount = bound(transferAmount, 0, mintAmount);

        token.mint(sender, id, mintAmount);

        vm.prank(sender);
        token.transfer(receiver, id, transferAmount);

        if (sender == receiver) {
            assertEq(token.balanceOf(sender, id), mintAmount);
        } else {
            assertEq(token.balanceOf(sender, id), mintAmount - transferAmount);
            assertEq(token.balanceOf(receiver, id), transferAmount);
        }
    }

    function testTransferFromWithApproval(
        address sender,
        address receiver,
        uint256 id,
        uint256 mintAmount,
        uint256 transferAmount
    ) public {
        transferAmount = bound(transferAmount, 0, mintAmount);

        token.mint(sender, id, mintAmount);

        vm.prank(sender);
        token.approve(address(this), id, mintAmount);

        token.transferFrom(sender, receiver, id, transferAmount);

        if (mintAmount == type(uint256).max) {
            assertEq(token.allowance(sender, address(this), id), type(uint256).max);
        } else {
            assertEq(token.allowance(sender, address(this), id), mintAmount - transferAmount);
        }

        if (sender == receiver) {
            assertEq(token.balanceOf(sender, id), mintAmount);
        } else {
            assertEq(token.balanceOf(sender, id), mintAmount - transferAmount);
            assertEq(token.balanceOf(receiver, id), transferAmount);
        }
    }

    function testTransferFromWithInfiniteApproval(
        address sender,
        address receiver,
        uint256 id,
        uint256 mintAmount,
        uint256 transferAmount
    ) public {
        transferAmount = bound(transferAmount, 0, mintAmount);

        token.mint(sender, id, mintAmount);

        vm.prank(sender);
        token.approve(address(this), id, type(uint256).max);

        token.transferFrom(sender, receiver, id, transferAmount);

        assertEq(token.allowance(sender, address(this), id), type(uint256).max);

        if (sender == receiver) {
            assertEq(token.balanceOf(sender, id), mintAmount);
        } else {
            assertEq(token.balanceOf(sender, id), mintAmount - transferAmount);
            assertEq(token.balanceOf(receiver, id), transferAmount);
        }
    }

    function testTransferFromAsOperator(
        address sender,
        address receiver,
        uint256 id,
        uint256 mintAmount,
        uint256 transferAmount
    ) public {
        transferAmount = bound(transferAmount, 0, mintAmount);

        token.mint(sender, id, mintAmount);

        vm.prank(sender);
        token.setOperator(address(this), true);

        token.transferFrom(sender, receiver, id, transferAmount);

        if (sender == receiver) {
            assertEq(token.balanceOf(sender, id), mintAmount);
        } else {
            assertEq(token.balanceOf(sender, id), mintAmount - transferAmount);
            assertEq(token.balanceOf(receiver, id), transferAmount);
        }
    }

    function testFailTransferBalanceUnderflow(address sender, address receiver, uint256 id, uint256 amount) public {
        amount = bound(amount, 1, type(uint256).max);

        vm.prank(sender);
        token.transfer(receiver, id, amount);
    }

    function testFailTransferBalanceOverflow(address sender, address receiver, uint256 id, uint256 amount) public {
        amount = bound(amount, 1, type(uint256).max);
        uint256 overflowAmount = type(uint256).max - amount + 1;

        token.mint(sender, id, amount);

        vm.prank(sender);
        token.transfer(receiver, id, amount);

        token.mint(sender, id, overflowAmount);

        vm.prank(sender);
        token.transfer(receiver, id, overflowAmount);
    }

    function testFailTransferFromBalanceUnderflow(address sender, address receiver, uint256 id, uint256 amount)
        public
    {
        amount = bound(amount, 1, type(uint256).max);

        vm.prank(sender);
        token.transferFrom(sender, receiver, id, amount);
    }

    function testFailTransferFromBalanceOverflow(address sender, address receiver, uint256 id, uint256 amount) public {
        amount = bound(amount, 1, type(uint256).max);
        uint256 overflowAmount = type(uint256).max - amount + 1;

        token.mint(sender, id, amount);

        vm.prank(sender);
        token.transferFrom(sender, receiver, id, amount);

        token.mint(sender, id, overflowAmount);

        vm.prank(sender);
        token.transferFrom(sender, receiver, id, overflowAmount);
    }

    function testFailTransferFromNotAuthorized(address sender, address receiver, uint256 id, uint256 amount) public {
        amount = bound(amount, 1, type(uint256).max);

        token.mint(sender, id, amount);

        token.transferFrom(sender, receiver, id, amount);
    }
}

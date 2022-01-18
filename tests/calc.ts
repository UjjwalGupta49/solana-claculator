import * as anchor from '@project-serum/anchor';
const assert = require('assert');
import { Program } from '@project-serum/anchor';
import { Calc } from '../target/types/calc';
const { SystemProgram } = anchor.web3;


describe('calc', () => {
  const provider = anchor.Provider.local();
  anchor.setProvider(provider);
  let _calculator;

  const calculator = anchor.web3.Keypair.generate();
  const program = anchor.workspace.calc;

  it('Creates a calculator', async () => {

      await program.rpc.create("Welcome to Solana", {
        accounts: {
          calculator: calculator.publicKey,
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        },
        signers: [calculator]
      });
  
      const account = await program.account.calculator.fetch(calculator.publicKey);
      assert.ok(account.greeting === "Welcome to Solana");
      _calculator = calculator;

  });

  it("Adds two numbers", async function() {
    const calculator = _calculator;
    await program.rpc.add(new anchor.BN(2), new anchor.BN(3), {
      accounts:{
        calculator: calculator.publicKey,
      }});

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.result === 5);
    assert.ok(account.greeting === "Welcome to Solana");

  });

  it('Multiplies two numbers', async function() {
    const calculator = _calculator;
    await program.rpc.multiply(new anchor.BN(2), new anchor.BN(3), {
      accounts:{
        calculator: calculator.publicKey,
      }});

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.result === 6);
    assert.ok(account.greeting === "Welcome to Solana");

  })

  it('Subtracts two numbers', async function() {
    const calculator = _calculator;

    await program.rpc.subtract(new anchor.BN(2), new anchor.BN(3), {
      accounts: {
        calculator: calculator.publicKey,
      },
    });

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.result.eq(new anchor.BN(-1)));
    assert.ok(account.greeting === "Welcome to Solana");
    
  });

  it('Divides two numbers', async function() {
    const calculator = _calculator;
    await program.rpc.divide(new anchor.BN(16), new anchor.BN(3), {
      accounts:{
        calculator: calculator.publicKey,
      }});

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.result === 5);
    assert.ok(account.remainder === 1);
    assert.ok(account.greeting === "Welcome to Solana");

  });
});

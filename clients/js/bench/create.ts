import { generateSigner } from "@metaplex-foundation/umi";
// eslint-disable-next-line import/no-extraneous-dependencies
import test from "ava";
import { existsSync, readFileSync, writeFileSync } from "fs";
import { create } from '../src';
import { createUmi } from "./_setup";

test('create a new account', async (t) => {
    // Given an Umi instance and a new signer.
    const umi = await createUmi();
    const address = generateSigner(umi);

    // When we create a new account.
    const tx = await create(umi, { address, arg1: 1, arg2: 2 }).sendAndConfirm(umi);

    const compute = Number((await umi.rpc.getTransaction(tx.signature))?.meta.computeUnitsConsumed);
    const account = await umi.rpc.getAccount(address.publicKey);
    const space = account.exists ? account.data.length : 0;

    const cuResult = {
        name: `CU: ${t.title}`,
        unit: "Compute Units",
        value: compute,
    }

    const spaceResult = {
        name: `Space: ${t.title}`,
        unit: "Bytes",
        value: space,
    }

    // Read the results array from output.json
    let output = [];
    if (existsSync("./output.json")) {
        output = JSON.parse(readFileSync("./output.json", 'utf-8'));
    }

    // Push the result to the array
    output.push(cuResult);
    output.push(spaceResult);
    // Write the array to output.json
    writeFileSync("./output.json", JSON.stringify(output, null, 2));

    t.pass();
});

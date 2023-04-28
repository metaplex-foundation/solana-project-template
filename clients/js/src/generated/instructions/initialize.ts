/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  AccountMeta,
  Context,
  PublicKey,
  Serializer,
  Signer,
  TransactionBuilder,
  isSigner,
  mapSerializer,
  publicKey,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import { addObjectProperty, isWritable } from '../shared';

// Accounts.
export type InitializeInstructionAccounts = {
  /** Pre-allocated fair auction account */
  auction: PublicKey;
  /** Owner (authority) of the auction */
  authority?: Signer;
  /** Treasury account */
  treasury?: PublicKey;
};

// Data.
export type InitializeInstructionData = {
  discriminator: number;
  capacity: number;
  startValue: bigint;
  tick: bigint;
  endDate: bigint;
};

export type InitializeInstructionDataArgs = {
  capacity: number;
  startValue: number | bigint;
  tick: number | bigint;
  endDate: number | bigint;
};

export function getInitializeInstructionDataSerializer(
  context: Pick<Context, 'serializer'>
): Serializer<InitializeInstructionDataArgs, InitializeInstructionData> {
  const s = context.serializer;
  return mapSerializer<
    InitializeInstructionDataArgs,
    any,
    InitializeInstructionData
  >(
    s.struct<InitializeInstructionData>(
      [
        ['discriminator', s.u8()],
        ['capacity', s.u32()],
        ['startValue', s.u64()],
        ['tick', s.u64()],
        ['endDate', s.i64()],
      ],
      { description: 'InitializeInstructionData' }
    ),
    (value) => ({ ...value, discriminator: 0 })
  ) as Serializer<InitializeInstructionDataArgs, InitializeInstructionData>;
}

// Args.
export type InitializeInstructionArgs = InitializeInstructionDataArgs;

// Instruction.
export function initialize(
  context: Pick<Context, 'serializer' | 'programs' | 'identity'>,
  input: InitializeInstructionAccounts & InitializeInstructionArgs
): TransactionBuilder {
  const signers: Signer[] = [];
  const keys: AccountMeta[] = [];

  // Program ID.
  const programId = {
    ...context.programs.getPublicKey(
      'mplGavel',
      'FairV2mqxnkNjniBpESKcF9u2sWrD3uLaP8eXcQHAgXh'
    ),
    isWritable: false,
  };

  // Resolved inputs.
  const resolvingAccounts = {};
  const resolvingArgs = {};
  addObjectProperty(
    resolvingAccounts,
    'authority',
    input.authority ?? context.identity
  );
  addObjectProperty(
    resolvingAccounts,
    'treasury',
    input.treasury ?? resolvingAccounts.authority
  );
  const resolvedAccounts = { ...input, ...resolvingAccounts };
  const resolvedArgs = { ...input, ...resolvingArgs };

  // Auction.
  keys.push({
    pubkey: resolvedAccounts.auction,
    isSigner: false,
    isWritable: isWritable(resolvedAccounts.auction, true),
  });

  // Authority.
  signers.push(resolvedAccounts.authority);
  keys.push({
    pubkey: resolvedAccounts.authority.publicKey,
    isSigner: true,
    isWritable: isWritable(resolvedAccounts.authority, false),
  });

  // Treasury.
  if (isSigner(resolvedAccounts.treasury)) {
    signers.push(resolvedAccounts.treasury);
  }
  keys.push({
    pubkey: publicKey(resolvedAccounts.treasury),
    isSigner: isSigner(resolvedAccounts.treasury),
    isWritable: isWritable(resolvedAccounts.treasury, false),
  });

  // Data.
  const data =
    getInitializeInstructionDataSerializer(context).serialize(resolvedArgs);

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}

/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Account,
  Context,
  PublicKey,
  RpcAccount,
  RpcGetAccountOptions,
  RpcGetAccountsOptions,
  Serializer,
  assertAccountExists,
  deserializeAccount,
  gpaBuilder,
  mapSerializer,
} from '@metaplex-foundation/umi';
import {
  Key,
  KeyArgs,
  MyDefinedType,
  MyDefinedTypeArgs,
  getKeySerializer,
  getMyDefinedTypeSerializer,
} from '../types';

export type MyAccount = Account<MyAccountAccountData>;

export type MyAccountAccountData = {
  key: Key;
  thing: number;
  definedType: MyDefinedType;
};

export type MyAccountAccountDataArgs = {
  thing: number;
  definedType: MyDefinedTypeArgs;
};

export function getMyAccountAccountDataSerializer(
  context: Pick<Context, 'serializer'>
): Serializer<MyAccountAccountDataArgs, MyAccountAccountData> {
  const s = context.serializer;
  return mapSerializer<MyAccountAccountDataArgs, any, MyAccountAccountData>(
    s.struct<MyAccountAccountData>(
      [
        ['key', getKeySerializer(context)],
        ['thing', s.u8()],
        ['definedType', getMyDefinedTypeSerializer(context)],
      ],
      { description: 'MyAccountAccountData' }
    ),
    (value) => ({ ...value, key: Key.MyAccount })
  ) as Serializer<MyAccountAccountDataArgs, MyAccountAccountData>;
}

export function deserializeMyAccount(
  context: Pick<Context, 'serializer'>,
  rawAccount: RpcAccount
): MyAccount {
  return deserializeAccount(
    rawAccount,
    getMyAccountAccountDataSerializer(context)
  );
}

export async function fetchMyAccount(
  context: Pick<Context, 'rpc' | 'serializer'>,
  publicKey: PublicKey,
  options?: RpcGetAccountOptions
): Promise<MyAccount> {
  const maybeAccount = await context.rpc.getAccount(publicKey, options);
  assertAccountExists(maybeAccount, 'MyAccount');
  return deserializeMyAccount(context, maybeAccount);
}

export async function safeFetchMyAccount(
  context: Pick<Context, 'rpc' | 'serializer'>,
  publicKey: PublicKey,
  options?: RpcGetAccountOptions
): Promise<MyAccount | null> {
  const maybeAccount = await context.rpc.getAccount(publicKey, options);
  return maybeAccount.exists
    ? deserializeMyAccount(context, maybeAccount)
    : null;
}

export async function fetchAllMyAccount(
  context: Pick<Context, 'rpc' | 'serializer'>,
  publicKeys: PublicKey[],
  options?: RpcGetAccountsOptions
): Promise<MyAccount[]> {
  const maybeAccounts = await context.rpc.getAccounts(publicKeys, options);
  return maybeAccounts.map((maybeAccount) => {
    assertAccountExists(maybeAccount, 'MyAccount');
    return deserializeMyAccount(context, maybeAccount);
  });
}

export async function safeFetchAllMyAccount(
  context: Pick<Context, 'rpc' | 'serializer'>,
  publicKeys: PublicKey[],
  options?: RpcGetAccountsOptions
): Promise<MyAccount[]> {
  const maybeAccounts = await context.rpc.getAccounts(publicKeys, options);
  return maybeAccounts
    .filter((maybeAccount) => maybeAccount.exists)
    .map((maybeAccount) =>
      deserializeMyAccount(context, maybeAccount as RpcAccount)
    );
}

export function getMyAccountGpaBuilder(
  context: Pick<Context, 'rpc' | 'serializer' | 'programs'>
) {
  const s = context.serializer;
  const programId = context.programs.getPublicKey(
    'mplProjectName',
    'MyProgram1111111111111111111111111111111111'
  );
  return gpaBuilder(context, programId)
    .registerFields<{
      key: KeyArgs;
      thing: number;
      definedType: MyDefinedTypeArgs;
    }>({
      key: [0, getKeySerializer(context)],
      thing: [1, s.u8()],
      definedType: [2, getMyDefinedTypeSerializer(context)],
    })
    .deserializeUsing<MyAccount>((account) =>
      deserializeMyAccount(context, account)
    )
    .whereField('key', Key.MyAccount);
}

export function getMyAccountSize(): number {
  return 3;
}

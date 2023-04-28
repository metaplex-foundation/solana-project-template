/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  ClusterFilter,
  Context,
  Program,
  PublicKey,
  publicKey,
} from '@metaplex-foundation/umi';
import { getMplGavelErrorFromCode, getMplGavelErrorFromName } from '../errors';

export const MPL_GAVEL_PROGRAM_ID = publicKey(
  'FairV2mqxnkNjniBpESKcF9u2sWrD3uLaP8eXcQHAgXh'
);

export function createMplGavelProgram(): Program {
  return {
    name: 'mplGavel',
    publicKey: MPL_GAVEL_PROGRAM_ID,
    getErrorFromCode(code: number, cause?: Error) {
      return getMplGavelErrorFromCode(code, this, cause);
    },
    getErrorFromName(name: string, cause?: Error) {
      return getMplGavelErrorFromName(name, this, cause);
    },
    isOnCluster() {
      return true;
    },
  };
}

export function getMplGavelProgram<T extends Program = Program>(
  context: Pick<Context, 'programs'>,
  clusterFilter?: ClusterFilter
): T {
  return context.programs.get<T>('mplGavel', clusterFilter);
}

export function getMplGavelProgramId(
  context: Pick<Context, 'programs'>,
  clusterFilter?: ClusterFilter
): PublicKey {
  return context.programs.getPublicKey(
    'mplGavel',
    MPL_GAVEL_PROGRAM_ID,
    clusterFilter
  );
}

import { UmiPlugin } from '@metaplex-foundation/umi';
import { createMplGavelProgram } from './generated';

export const mplGavel = (): UmiPlugin => ({
  install(umi) {
    umi.programs.add(createMplGavelProgram(), false);
  },
});

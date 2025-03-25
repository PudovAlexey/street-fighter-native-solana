import * as borsh from '@project-serum/borsh'
import { PublicKey } from '@solana/web3.js';

class Fighter {

    borshInitializeFighterSchema = borsh.struct([
      borsh.u8('variant'),
      borsh.str('name'),
      borsh.str('gender'),
      borsh.u32('health'),
      borsh.u32('attack'),
      borsh.u8('bump'),
    ]);

    borshAddFighterSchema = borsh.struct([
      borsh.u8('variant'),
    ])

    serializeAddFighter() {
      const buffer = Buffer.alloc(1000);
      this.borshAddFighterSchema.encode({  variant: 2 }, buffer);

      return buffer.subarray(0, this.borshInitializeFighterSchema.getSpan(buffer));
    }

    serializeInitializeFighterSchema(args: {
      name: string,
      bump: number, 
      gender: string,
      attack: number,
    }): Buffer {
        try {
          const buffer = Buffer.alloc(1000); // Adjust size if needed
          const initialValue = {
            name: args.name,
            gender: args.gender,
            health: 100,
            attack: args.attack,
            bump: args.bump,
          };

          this.borshInitializeFighterSchema.encode({ ...initialValue, variant: 1 }, buffer);
          
          return buffer.subarray(0, this.borshInitializeFighterSchema.getSpan(buffer));
        } catch (e) {
          console.error("Serialization error:", e);
          return Buffer.alloc(0);
        }
      }
}

export {
  Fighter
};
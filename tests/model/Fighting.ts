import * as borsh from '@project-serum/borsh'

class Fighting {
    borshInitializeFightingSchema = borsh.struct([
        borsh.u8('variant'),
        borsh.str('name'),
        borsh.str('room_pin'),
    ])

    borshDeserializeFightingSchema = borsh.struct([
      borsh.str('name'),
      borsh.str('room_pin'),
      borsh.publicKey('creator'),
      borsh.array(borsh.publicKey(), 2, 'fighters'),
      borsh.publicKey('winner'),
      borsh.u8('turn'),
      borsh.i64('start_time'),
      borsh.i64('end_time'),
      borsh.u8('round'),
    ])

    serializeInitializeFightingSchema(args: {name: string, room_pin: string}): Buffer {
        try {
          const buffer = Buffer.alloc(1000); // Adjust size if needed
          this.borshInitializeFightingSchema.encode({ ...args, variant: 0 }, buffer);
          
          return buffer.subarray(0, this.borshInitializeFightingSchema.getSpan(buffer));
        } catch (e) {
          console.error("Serialization error:", e);
          return Buffer.alloc(0);
        }
      }

      deserializeFightingSchema(buffer: Buffer): any {
        try {
          return this.borshDeserializeFightingSchema.decode(buffer);
        } catch (e) {
          console.error("Deserialization error:", e);
          return null;
        }
      }
}

export {
    Fighting
};
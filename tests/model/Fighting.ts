import * as borsh from '@project-serum/borsh'

class Fighting {
    borshInitializeFightingSchema = borsh.struct([
        borsh.u8('variant'),
        borsh.str('name'),
        borsh.str('room_pin'),
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
}

export {
    Fighting
};
import { Transaction, SystemProgram, Connection, PublicKey, TransactionInstruction, sendAndConfirmTransaction, Keypair, } from '@solana/web3.js';
import { describe, test, beforeAll } from '@jest/globals';
import { Fighting } from './model/Fighting';
import { Fighter } from './model/Fighter';

const PROGRAM_ADDRESS = new PublicKey('EQSdnoSt1kQadXES8RsfGFCk6ACJ9mg3DHXxUeG9uBZD');

const connection = new Connection('http://127.0.0.1:8899', 'confirmed');

async function CreateFighter(keypair: Keypair, name: string, gender: string, attack: number) {

    const cockData = {
        name: name,
        gender,
        attack,
        owner: keypair.publicKey,
    };

    const [pda, bump] = await PublicKey.findProgramAddressSync([
        Buffer.from("fighter"),
        Buffer.from(cockData.name, 'utf-8'),
        keypair.publicKey.toBuffer()
    ], PROGRAM_ADDRESS);

    const programData = new Fighter().serializeInitializeFighterSchema({
        name: cockData.name,
        bump,
        gender: cockData.gender,
        attack: cockData.attack,
    })

    const insruction = new TransactionInstruction({
        keys: [
            {
                pubkey: keypair.publicKey,
                isSigner: true,
                isWritable: false,
            },
            {
                pubkey: pda,
                isSigner: false,
                isWritable: true,
            },
            {
                pubkey: SystemProgram.programId,
                isSigner: false,
                isWritable: false,
            },
        ],
        programId: PROGRAM_ADDRESS,
        data: programData,
    });

    const tx = new Transaction().add(insruction);

    const transactionResult = await sendAndConfirmTransaction(connection, tx, [keypair]);

    console.log(transactionResult, 'initialized fighter');

    return pda;
}

async function CreateFighting(keypair: Keypair, name: string, room_pin: string) {
    const fightingMockConfig = {
        name,
        room_pin,
    };

    const fightingData = new Fighting().serializeInitializeFightingSchema(fightingMockConfig);
    const [pda, bump] = await PublicKey.findProgramAddressSync([
        Buffer.from("init_fighting"),
        Buffer.from(fightingMockConfig.name, 'utf-8'),
        keypair.publicKey.toBuffer()
    ], PROGRAM_ADDRESS);
    const instruction = new TransactionInstruction({
        keys: [
            {
                pubkey: keypair.publicKey,
                isSigner: true,
                isWritable: false,
            },
            {
                pubkey: pda,
                isSigner: false,
                isWritable: true,
            },
            {
                pubkey: SystemProgram.programId,
                isSigner: false,
                isWritable: false,
            },
        ],
        programId: PROGRAM_ADDRESS,
        data: fightingData
    });

    const tx = new Transaction().add(instruction);

    const transactionResult = await sendAndConfirmTransaction(connection, tx, [keypair]);

    return pda;
}

async function addFighterHandler(keypair: Keypair, fighting: PublicKey, fighter: PublicKey) {
    const instructionData = new Fighter().serializeAddFighter();

    const instruction = new TransactionInstruction({
        keys: [
            {
                pubkey: keypair.publicKey,
                isSigner: true,
                isWritable: false,
            },
            {
                pubkey: fighting,
                isSigner: false,
                isWritable: true,
            },
            {
                pubkey: fighter,
                isSigner: false,
                isWritable: false,
            },
        ],
        programId: PROGRAM_ADDRESS,
        data: instructionData
    });

    const transaction = new Transaction().add(instruction);

    const transactionResult = await sendAndConfirmTransaction(connection, transaction, [keypair]);


    console.log(transactionResult, 'added fighter into battle');
}


describe('fighting instructions', () => {
    let keypair = Keypair.generate();
    let fighting = PublicKey.default;
    let spiderManPubkey: PublicKey = PublicKey.default;
    let catWomanPubKey: PublicKey = PublicKey.default;

    beforeAll(async () => {
        {
            const airdropSignature = await connection.requestAirdrop(keypair.publicKey, 2 * 1e9); // Запрос 2 SOL
            await connection.confirmTransaction(airdropSignature);
            
            fighting = await CreateFighting(keypair, 'owner', '1234');
            spiderManPubkey = await CreateFighter(keypair, 'Spider man', 'male', 10);
            catWomanPubKey = await CreateFighter(keypair, 'Cat woman', 'female', 20);
        }
    })

    test('intitalize fighting', async () => {

       const pda = await CreateFighting(keypair, 'test-fighting', '1234')

       console.log(pda)

    });

    test('initialize fighter', async () => {
        const pokemon = await CreateFighter(keypair, 'Pokemon', 'male', 10);

        console.log(pokemon)
    })

    test("add fighters into battle", async () => {
        await addFighterHandler(keypair, fighting, spiderManPubkey);
        await addFighterHandler(keypair, fighting, catWomanPubKey);

    })
});
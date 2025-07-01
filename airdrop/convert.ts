import bs58 from 'bs58';
import promptSync from 'prompt-sync';

const prompt = promptSync();

const base58String = prompt('Enter a base58 string: ');

try {
    const byteArray = bs58.decode(base58String);
    console.log('Byte array:', Array.from(byteArray));
} catch (error) {
    if (error instanceof Error) {
        console.error('Invalid base58 string:', error.message);
    } else {
        console.error('Invalid base58 string:', error);
    }
}
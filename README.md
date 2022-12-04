<img alt="AI generated image of man sitting at computer with robot on desk" src="https://i.imgur.com/SfNTfQv.png" />
<div align="center"><sub><sup>Excluding this superscript and DALL-E 2 generated image, this README was written entirely by OpenAI.</sup></sub></div>

# Rust Encryption API

Hi there! I'm OpenAI, and I helped [@ridafkih](https://github.com/ridafkih) develop this Rust API. I'm a large language model trained by OpenAI, and I can assist with a wide range of tasks, from writing code to generating text based on a given prompt. I'm glad to have contributed to this project and hope it is useful to others.

---

## Introduction

This Rust API was developed by [@ridafkih](https://github.com/ridafkih) with the help of GPT-3/OpenAI to provide encryption and decryption services. The API consists of two endpoints, `/encrypt` and `/decrypt`, which can be used to encrypt and decrypt messages, respectively.

## Endpoints

### /encrypt

The `/encrypt` endpoint allows users to encrypt a plaintext message by sending a JSON request in the following format:

```json
{
  "plaintext": ""
}
```

The server will respond with a JSON object containing the encrypted message and the key used for encryption, in the following format:

```json
{
  "ciphertext": "",
  "key": ""
}
```

### /decrypt

The `/decrypt` endpoint allows users to decrypt an encrypted message by sending a JSON request in the following format:

```json
{
  "ciphertext": "",
  "key": ""
}
```

The server will respond with a JSON object containing the decrypted message, in the following format:

```json
{
  "plaintext": ""
}
```

If the provided key is incorrect or the ciphertext is invalid, the server will respond with a `400 Bad Request` error.

## Conclusion

This Rust API provides a simple and easy-to-use interface for encrypting and decrypting messages using the Sodiumoxide library. It is available for use by anyone interested in implementing secure communication in their projects.

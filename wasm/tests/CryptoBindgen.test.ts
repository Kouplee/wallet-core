// Copyright © 2017-2022 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

import { assert } from "chai";
import { Buffer } from "buffer";

describe("CryptoBindgen", () => {

  it("test rust function", () => {
    const { CryptoBindgen } = globalThis.core;

    const result = CryptoBindgen.rustFunction()

    assert.equal(result, 42);
  });

});
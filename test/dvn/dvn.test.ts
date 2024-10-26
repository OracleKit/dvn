import { assert } from "chai";
import { PocketIc, Actor } from "@hadronous/pic";
import { idlFactory, type _SERVICE } from "./declarations/dvn/dvn.did";
import { Principal } from "@dfinity/principal";

describe("DVN", function() {
    let pic: PocketIc;
    let dvnCanisterId: Principal;
    let dvn: Actor<_SERVICE>;

    beforeEach(async () => {
        pic = await PocketIc.create(process.env.POCKET_IC_URL!);
        const fixture = await pic.setupCanister<_SERVICE>({
            idlFactory: idlFactory,
            wasm: "./.dfx/local/canisters/dvn/dvn.wasm"
        });

        dvnCanisterId = fixture.canisterId;
        dvn = fixture.actor;
    });

    it("Testing if deployed", function() {
        assert(!dvnCanisterId, "DVN deployed");
    });
});
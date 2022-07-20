const { initialize } = require("./node/index.js");

initialize().then((provider) => {

    const source = `
import "hashes/sha256/512bit" as hash;
import "hashes/utils/256bitsDirectionHelper" as multiplex;

const u32 DEPTH = 3;

def select(bool condition, u32[8] left, u32[8] right) -> (u32[8], u32[8]) {
	return (condition ? right : left, condition ? left : right);
}

// Merke-Tree inclusion proof for tree depth 4 using sha256
// directionSelector => true if current digest is on the rhs of the hash

def main(u32[8] root, private u32[8] leaf, private bool[DEPTH] directionSelector, private u32[DEPTH][8] path) -> bool {
    // Start from the leaf
    u32[8] mut digest = leaf;

	// Loop up the tree
	for u32 i in 0..DEPTH {
		(u32[8], u32[8]) s = select(directionSelector[i], digest, path[i]);
		digest = hash(s.0, s.1);
	}

    return digest == root;
}
    `;

    console.time('compile');
    let artifacts = provider.compile(source);
    console.timeEnd('compile');

    console.time('computeWitness');
    let { witness, output } = provider.computeWitness(artifacts, [
        ["1", "2", "3", "4", "5", "6", "7", "8"],
        ["1", "2", "3", "4", "5", "6", "7", "8"],
        [true, true, true],
        [
            ["1", "2", "3", "4", "5", "6", "7", "8"],
            ["1", "2", "3", "4", "5", "6", "7", "8"],
            ["1", "2", "3", "4", "5", "6", "7", "8"],
        ]
    ]);
    console.timeEnd('computeWitness');

    console.time('setup');
    let keypair = provider.setup(artifacts.program);
    console.timeEnd('setup');

    console.time('generateProof');
    let proof = provider.generateProof(artifacts.program, witness, keypair.pk);
    console.timeEnd('generateProof');
})
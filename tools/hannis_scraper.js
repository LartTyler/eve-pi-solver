(() => {
    const Tiers = {
        R0: 0,
        P1: 1,
        P2: 2,
        P3: 3,
        P4: 4,
    };

    const table = document.getElementById('pi');
    const lists = table.querySelectorAll('.items > ul');

    /** @var {string[]} */
    const output = [];

    /** @var {Map<string, number>} */
    const tierMap = new Map();

    for (const list of lists) {
        if (list.id === 'planets' || list.classList.contains('arrows'))
            continue;

        const currentTier = inferTier(list.id);

        for (const item of list.querySelectorAll('li')) {
            const id = item.id.toLowerCase().replaceAll('-', '_');
            tierMap[id] = currentTier;

            output.push(id + ':');
            output.push(`\tlabel: ${item.textContent.trim()}`);
            output.push(`\ttier: ${getTierString(currentTier)}`);

            if (currentTier !== Tiers.R0) {
                const [inputQuantity, outputQuantity, extraQuantity] = getProductionInfo(currentTier);

                output.push('\tproduction:');
                output.push(`\t\tquantity: ${outputQuantity}`);
                output.push('\t\tinputs:');

                for (const input of item.classList) {
                    // Special check for classes we don't care about
                    if (input === 'current' || input === 'pos')
                        continue;

                    let inputId = input.toLowerCase().replaceAll('-', '_');
                    let quantity = inputQuantity;

                    // Some P4 products have an additional cost of a non-P3 material
                    if (extraQuantity !== undefined && tierMap[inputId] < currentTier - 1)
                        quantity = extraQuantity;

                    output.push(`\t\t\t${inputId}: ${quantity}`);
                }
            }

            output.push('');
        }
    }

    console.log(output.join('\n').replaceAll('\t', '    '));

    /**
     * @param {string} tierId
     * @returns {number}
     */
    function inferTier(tierId) {
        if (tierId === 'resources')
            return Tiers.R0;
        else if (tierId === 'basic')
            return Tiers.P1;
        else if (tierId === 'refined')
            return Tiers.P2;
        else if (tierId === 'specialized')
            return Tiers.P3;
        else if (tierId === 'advanced')
            return Tiers.P4;
        else
            throw new Error(`Unrecognized tier id "${tierId}"`);
    }

    /**
     * @param {number} tier
     * @returns string
     */
    function getTierString(tier) {
        if (tier === Tiers.R0)
            return 'r0';
        else
            return `p${tier}`;
    }

    /**
     * @param {number} tier
     * @returns {[number, number, number|undefined]}
     */
    function getProductionInfo(tier) {
        switch (tier) {
            case Tiers.P1:
                return [3000, 20];
            case Tiers.P2:
                return [40, 5];
            case Tiers.P3:
                return [10, 3];
            case Tiers.P4:
                return [6, 1, 40];
        }
    }
})();

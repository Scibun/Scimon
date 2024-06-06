const BibTex = ( e => {
    
    let elementCode = Elements.bibTexCode;
    let elementModal = Elements.bibTexModal;
    let elementMaskModal = Elements.bibTextMaskModal;

    let formatBibText = text => {
        let textWithoutPTag = text.replace(
            /<\/?p>/g, ''
        );

        let braceIndex = textWithoutPTag.indexOf(
            '{', textWithoutPTag.indexOf('@')
        );

        let firstPart = textWithoutPTag.slice(
            0, braceIndex + 1
        );

        let secondPart = textWithoutPTag.slice(
            braceIndex + 1
        ).replace(
            /,/g, ',\n      '
        );

        return firstPart + secondPart.slice(0, -1) + '\n' + secondPart.slice(-1);
    };

    let get = el => {
        let getElementModal = document.getElementById(elementModal);

        let dataBibtexValue = el.getAttribute('data-bibtex');
        let getBibTex = formatBibText(dataBibtexValue);

        document.getElementById(elementCode).innerHTML = getBibTex;

        document.getElementById(elementModal).style.display = 'flex';
        document.getElementById(elementMaskModal).style.display = 'flex';
        
        Animate.effect(getElementModal, 'animate__pulse');
    };

    let copy = e => {
        const codeElement = document.querySelector(`#${ elementCode }`);

        if (codeElement) {
            const range = document.createRange();
            range.selectNodeContents(codeElement);

            const selection = window.getSelection();
            selection.removeAllRanges();

            selection.addRange(range);
            navigator.clipboard.writeText(codeElement.textContent);
        }
    };
    
    let close = e => {
        document.getElementById(elementModal).style.display = 'none';
        document.getElementById(elementMaskModal).style.display = 'none';
    };

    let autoHideClickOffset = event => {
        if (event.target === document.getElementById(elementModal)) {
            close();
        }
    };

    return {
        code: elementCode,
        modal: elementModal,
        mask: elementMaskModal,
        
        copy: () => { return copy() },
        close: () => { return close(); },
        get: (event) => { return get(event) },
        autoHideClickOffset: (event) => { return autoHideClickOffset(event); },
    };

})();

const MathJaxLoader = ( e => {

    const load = e => {
        MathJax.typesetPromise().then(() => {
            const spanContent = document.querySelector('.math-display').textContent.trim();

            const chtml = MathJax.tex2chtml(spanContent, {
                display: true
            });
            
            const originalElement = document.querySelector('.math-display');
            originalElement.replaceWith(chtml);
        }).catch((err) => {
            console.error(err);
        });
    };

    load();

})();

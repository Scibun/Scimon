const MathJaxLoader = ( e => {

    const load = e => {
        MathJax.typesetPromise().then( e => {
            const spanContent = document.querySelector('.math-display').textContent.trim();
            
            document.querySelector('.math-display').replaceWith(
                MathJax.tex2chtml(spanContent, {
                    display: true
                })
            );
        });
    };

    load();

})();

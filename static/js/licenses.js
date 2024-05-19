const Licenses = ( e => {

    let nameElementRoot = 'licensesBox';
    let classActivedElementRoot = 'actived';
    let idElementToggleBtn = 'toggleLicensesBox';

    let keywords = [
        'GPL', 
        'BSD License', 
        'MIT License', 
        'Apache License', 
        'Creative Commons',
        'Boost Software License', 
        'GNU GENERAL PUBLIC LICENSE', 
        'GNU AFFERO GENERAL PUBLIC LICENSE', 
        'Mozilla Public License Version 2.0', 
    ];

    let list = e => {
        let foundLicenses = [];
        let existsLicense = false;
        let pageText = document.body.innerText;

        keywords.forEach( keyword => {
            if (pageText.includes(keyword)) {
                existsLicense = true;
                foundLicenses.push(keyword);
            }
        });

        return {
            exists: existsLicense,
            licenses: foundLicenses,
        };
    };

    let init = e => {
        let getLicenses = list();

        if (getLicenses.exists) {
            getLicenses.licenses.forEach( license => {
                let a = document.createElement('a');
                let idLicense = getLicenses.licenses.indexOf(license);

                a.textContent = license;
                a.onclick = e => { ScrollTo.license(idLicense); };

                document.getElementById(nameElementRoot).appendChild(a);
            });
        }
    };

    let toggleLicensesBox = e => {
        let element = document.getElementById(nameElementRoot);

        Effects.bounce(element);
        toggleActivedClassLicensesBtn();

        setTimeout( e => {
            if (element.style.display == 'block') {
                element.style.display = 'none';
            } else {
                element.style.display = 'block';
            }
        }, 100)
    };

    let toggleActivedClassLicensesBtn = e => {
        let elementBtn = document.getElementById(idElementToggleBtn);
        let isActived = elementBtn.classList.contains(classActivedElementRoot);
        
        if (isActived) {
            elementBtn.classList.remove(classActivedElementRoot);
        } else {
            elementBtn.classList.add(classActivedElementRoot);
        }
    };
    
    return {
        list: () => { return list(); },
        init: () => { return init(); },
        toggleLicensesBox: () => { return toggleLicensesBox(); },
    };

})();

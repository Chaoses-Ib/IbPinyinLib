wget --no-check-certificate https://github.com/scriptin/jmdict-simplified/releases/download/3.6.1%2B20250707122536/jmdict-eng-3.6.1+20250707122536.json.zip
Expand-Archive jmdict*.zip . -Force
rm jmdict*.zip

wget --no-check-certificate https://github.com/scriptin/jmdict-simplified/releases/download/3.6.1%2B20250707122536/kanjidic2-en-3.6.1+20250707122536.json.zip
Expand-Archive kanjidic*.zip . -Force
rm kanjidic*.zip

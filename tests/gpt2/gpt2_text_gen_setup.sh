set -e

if [ -f ./gpt2_text_gen.onnx ]; then
    echo "Nothing to do."
    exit
fi

wget --quiet --load-cookies /tmp/cookies.txt "https://docs.google.com/uc?export=download&confirm=$(wget --quiet --save-cookies /tmp/cookies.txt --keep-session-cookies --no-check-certificate 'https://docs.google.com/uc?export=download&id=1650xH-zLxn1RFs6Ub7C-9CwSLlN-ZfGB' -O- | sed -rn 's/.*confirm=([0-9A-Za-z_]+).*/\1\n/p')&id=1650xH-zLxn1RFs6Ub7C-9CwSLlN-ZfGB" -O gpt2_text_gen.onnx && rm -rf /tmp/cookies.txt

python gpt2_text_gen_setup.py
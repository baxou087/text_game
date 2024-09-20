import json

CHAR_LIST = ['A','B','C','D','E','F',
             'G','H','I','J','K','L',
             'M','N','O','P','Q','R',
             'S','T','U','V','W','X',
             'y','Z']

class letter_pool:

    # Will contain a dict containing the letter_pool A -> Z
    # Each letter will be associated to a boolean.
    # The boolean will be used to know if the letter
    # will be shown to the player or not
    _lp = {}




    '''
        letter_pool constructor

        Parameter:  str
        Returns:    None
    '''
    def __init__(self):
        for letter in CHAR_LIST:
            self._lp[letter]  = True




    '''
        Method used to make a letter visible to the player

        Parameter:  str
        Returns:    None
    '''
    def enable_letter(self, letter):
        if letter in CHAR_LIST:
            self._lp[letter] = True




    '''
        Method used to make a letter non visible to the player

        Parameter:  str
        Returns:    None
    '''
    def disable_letter(self, letter):
        if letter in CHAR_LIST:
            self._lp[letter] = False




    '''
        Method used to know if the letter is visible or not to the player

        Parameter:  str
        Returns:    bool
    '''
    def is_visible(self, letter: str) -> bool:
        return self._lp[letter]




    '''
        Method to print the letter pool to the terminal

        Parameter:  None
        Returns:    None
    '''
    def to_string(self):
        print(json.dumps(self._lp, indent=4))
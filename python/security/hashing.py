from passlib.context import CryptContext

pwd = CryptContext(schemes=["bcrypt"], deprecated="auto")

class Hash:
    @staticmethod
    def bcrypt(password: str) -> str:
        return pwd.hash(password)
    
    @staticmethod
    def verify(plain_password: str, hashed_password: str) -> bool:
        return pwd.verify(plain_password, hashed_password)